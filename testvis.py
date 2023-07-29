import sys
import numpy as np
from vispy import app, scene, color
from vispy.visuals.transforms import STTransform



sizex: int = 128
sizey: int = 128
sizez: int = 128

def load_data(filename: str) -> np.ndarray:
    '''This method loads the data from the file and returns it as a numpy array.
       The data is stored as a binary file, and the data type is float32.
       In particular we are talking about a 3D array of shape (n, n, n).
       We infer the shape of the array from the size of the file. [ Size of the file = n * n * n * 4 ]
    '''
    with open(filename, 'rb') as f:
        global sizex, sizey, sizez
        sizex = int.from_bytes(f.read(4), byteorder='little')
        f.seek(4, 1)
        sizey = int.from_bytes(f.read(4), byteorder='little')
        f.seek(4, 1)
        sizez = int.from_bytes(f.read(4), byteorder='little')
        f.seek(4, 1)
        data = np.fromfile(f, dtype=np.float32)
        data = data.reshape((sizez, sizey, sizex))
        data = np.transpose(data, (1, 2, 0)) # yzx -> xyz
        # data = np.flip(data, axis=0) # flip x, old y, axis
    return data


vol = load_data('power_grid_db.bin')
# vol = np.log(vol)
# Prepare canvas
canvas: scene.SceneCanvas = scene.SceneCanvas(keys='interactive', show=True)
grid = canvas.central_widget.add_grid()
grid.spacing = 0
view = canvas.central_widget.add_view()

grid.add_widget(view, 0, 0, col_span=2)

# Create the volume visual for plane rendering
plane = scene.visuals.Volume(
    vol,
    parent=view.scene,
    raycasting_mode='plane',
    method='mip',
    plane_thickness=3.0,
    plane_position=(sizex, sizey, sizez),
    plane_normal=(1, 0, 0),
)

volume = scene.visuals.Volume(
    vol,
    parent=view.scene,
    raycasting_mode='volume',
    method='mip',
)
volume.set_gl_state('additive')
volume.opacity = 0.25

plane.cmap = color.colormap.HSL(ncolors=6, hue_start=0, saturation=1.0, value=1.0, controls=None, interpolation='linear')
volume.cmap = color.colormap.HSL(ncolors=6, hue_start=0, saturation=1.0, value=1.0, controls=None, interpolation='linear')

# Add also a scale for reference as color -> value
cbar = scene.ColorBarWidget(
    cmap=plane.cmap,
    label_color='white',
    clim=(vol.min(), vol.max()),
    orientation='right',
    border_width=1,
    border_color='white'
)

grid.add_widget(cbar, 0, 2)


# Create a camera
cam = scene.cameras.TurntableCamera(
    parent=view.scene, fov=60.0, azimuth=-42.0, elevation=30.0
)
view.camera = cam

# Create an XYZAxis visual
axis = scene.visuals.XYZAxis(parent=view)
s = STTransform(translate=(sizex, sizey), scale=(sizex, sizey, sizez, 1))
affine = s.as_matrix()
axis.transform = affine


def update_axis_visual(ax) -> None:
    """Sync XYZAxis visual with camera angles"""
    ax.transform.reset()
    ax.transform.rotate(cam.roll, (0, 0, 1))
    ax.transform.rotate(cam.elevation, (1, 0, 0))
    ax.transform.rotate(cam.azimuth, (0, 1, 0))
    ax.transform.scale((sizex, sizey, sizez))
    ax.transform.translate((sizex / 2, sizey / 2, sizez / 2))
    ax.update()

update_axis_visual(axis)


@canvas.events.mouse_move.connect
def on_mouse_move(event) -> None:
    # calculate the voxel we are hovering over
    if event.button == 1 and event.is_dragging:
        update_axis_visual(axis)


# Implement key presses
@canvas.events.key_press.connect
def on_key_press(event) -> None:
    if event.text == '1':
        methods = ['mip', 'average']
        method = methods[(methods.index(plane.method) + 1) % 2]
        print("Volume render method: %s" % method)
        plane.method = method
    elif event.text == '2':
        modes = ['volume', 'plane']
        if plane.raycasting_mode == modes[0]:
            plane.raycasting_mode = modes[1]
            print(modes[1])
        else:
            plane.raycasting_mode = modes[0]
            print(modes[0])
    elif event.text != '' and event.text in '{}':
        t = -1 if event.text == '{' else 1
        plane.plane_thickness += t
        plane.plane_thickness += t
        print(f"plane thickness: {plane.plane_thickness}")
    elif event.text != '' and event.text in '[]':
        shift = plane.plane_normal / np.linalg.norm(plane.plane_normal)
        if event.text == '[':
            plane.plane_position -= 2 * shift
        elif event.text == ']':
            plane.plane_position += 2 * shift
        print(f"plane position: {plane.plane_position}")
    elif event.text == 'x':
        plane.plane_normal = [0, 0, 1]
    elif event.text == 'y':
        plane.plane_normal = [0, 1, 0]
    elif event.text == 'z':
        plane.plane_normal = [1, 0, 0]
    elif event.text == 'o':
        plane.plane_normal = [1, 1, 1]
    elif event.text == ' ':
        if timer.running:
            timer.stop()
        else:
            timer.start()


def move_plane(event) -> None:
    z_pos: int = plane.plane_position[0]
    if z_pos < 0:
        plane.plane_position = plane.plane_position + [1, 0, 0]
    elif 0 < z_pos <= sizez:
        plane.plane_position = plane.plane_position - [1, 0, 0]
    else:
        plane.plane_position = (sizex, sizey, sizez)


timer: app.Timer = app.Timer('auto', connect=move_plane, start=True)

if __name__ == '__main__':
    canvas.show()
    if sys.flags.interactive == 0:
        plane.plane_position = (sizex, sizey, sizez)
        app.run()
