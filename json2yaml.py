'''This module loads a Three.JS JSON Scene file and converts it to my Scene description format file, which uses YAML, for my RayTracer.'''

import math
import json
import yaml
import numpy as np


def load_json_file(filename: str = "scene.json") -> dict:
    '''Loads a JSON file and returns it as a dictionary.'''
    with open(filename, 'r', encoding="utf-8") as file:
        return json.load(file)


def convert_json_to_yaml(json_dict: dict) -> str:
    '''Converts a JSON dictionary to a YAML string.'''
    if "object" not in json_dict or "type" not in json_dict["object"] or json_dict["object"]["type"] != "Scene":
        raise ValueError("JSON file is not a Three.JS JSON Scene file.")

    # * First off, we write the default values inside of the YAML file.
    yaml_dict: dict = {
        "constants": {
            "width": 800,
            "height": 800,
            "samplesPerPixel": 32,
            "maxDepth": 5000,
            "minDepth": 5,
            "environmentIntensity": 0.25,
            "sourcesLambda": 0.122364268571,
            "powerRenderCenter": [0.0, 0.0, 0.0]
        },
        "camera": {
            "lookFrom": [0.0, 0.0, 0.0],
            "lookAt": [0.0, 0.0, 0.0],
            "vup": [0.0, 1.0, 0.0],
            "vfov": 90.0,
            "aspectRatio": 1.0,
            "aperture": 0.1,
            "focusDistance": 10.0
        },
        "world": []
    }
    geoarr = json_dict["geometries"]
    geometries = {}
    for geometry in geoarr:
        geometries[geometry["uuid"]] = geometry
    matarr = json_dict["materials"]
    materials = {}
    for material in matarr:
        materials[material["uuid"]] = material
    objs = json_dict["object"]["children"]
    for obj in objs:
        new_obj = { "objType": obj["name"] }
        if new_obj["objType"] == "Sphere":
            # ? Sphere Parsing
            new_obj["center"] = [float(v) for v in obj["matrix"][12:15]]
            new_obj["radius"] = float(geometries[obj["geometry"]]["radius"])
        elif new_obj["objType"] == "Plane":
            # ? Planes Parsing
            new_obj["position"] = obj["matrix"][12:15]
            new_obj["width"] = float(geometries[obj["geometry"]]["width"])
            new_obj["height"] = float(geometries[obj["geometry"]]["height"])
            # TODO: Check the following code...
            # * If the rotation is 0 in each axis, then it's a XY plane.
            matrix4x4 = np.array(obj["matrix"]).reshape(4, 4)
            roll = (math.acos(matrix4x4[1][1]) * 57.2958) % 180.0
            pitch = (math.acos(matrix4x4[0][0]) * 57.2958) % 180.0
            if roll <= 0.01 and pitch <= 0.01:
                new_obj["objType"] = "XYRectangle"
            # * If the rotation is 90 in the X axis, then it's a YZ plane.
            elif roll - 90.0 <= 0.01 and pitch <= 0.01:
                new_obj["objType"] = "XZRectangle"
            # * If the rotation is 90 in the Y axis, then it's a XZ plane.
            elif roll <= 0.01 and pitch - 90.0 <= 0.01:
                new_obj["objType"] = "YZRectangle"
            elif roll - 90.0 <= 0.01 and pitch - 90.0 <= 0.01:
                new_obj["objType"] = "YZRectangle"
            else:
                raise ValueError("Plane has an invalid rotation.")
        elif new_obj["objType"] == "Box":
            # ? Box Parsing
            new_obj["position"] = [float(v) for v in obj["matrix"][12:15]]
            new_obj["width"] = float(geometries[obj["geometry"]]["width"])
            new_obj["height"] = float(geometries[obj["geometry"]]["height"])
            new_obj["depth"] = float(geometries[obj["geometry"]]["depth"])
        elif new_obj["objType"] == "PerspectiveCamera":
            # ? PerspectiveCamera Parsing
            # we save this data inside of the camera object
            yaml_dict["camera"]["lookFrom"] = [float(v) for v in obj["matrix"][12:15]]
            # TODO: Find the lookAt point in space which is positioned at the value t = focusDistance on the line vector the camera points to.
            # we find the vector the camera points to using the matrix
            yaml_dict["camera"]["lookAt"] = [float(v) for v in obj["matrix"][8:11]]
            yaml_dict["camera"]["vfov"] = float(obj["fov"])
            yaml_dict["camera"]["focusDistance"] = float(obj["focus"])
            continue
        elif new_obj["objType"] == "PointLight":
            # ? PointLight Parsing
            new_obj["objType"] = "Sphere"
            new_obj["center"] = [float(v) for v in obj["matrix"][12:15]]
            new_obj["radius"] = 1.0
            new_obj["material"] = {
                "matType": "DiffuseLight",
                "texType": "SolidColor",
                "texture": { "albedo": [1.0, 1.0, 1.0] },
                "intensity": float(obj["intensity"]) * 10.0
            }
        else:
            raise ValueError("Unknown object type.")
        # Now we set the material for each object
        if "material" in new_obj:
            yaml_dict["world"].append(new_obj)
            continue
        # color is in decimal, transform to r, g, b
        hex_color = hex(materials[obj["material"]]["color"])[2:]
        # hex to rgb
        rgbcolor = [int(hex_color[i:i + len(hex_color) // 3], 16) for i in range(0, len(hex_color), len(hex_color) // 3)]
        new_obj["material"] = {
            "matType": "Lambertian",
            "texType": "SolidColor",
            "texture": { "albedo": [float(c) / 255.0 for c in rgbcolor] }
        }
        yaml_dict["world"].append(new_obj)
    #return yaml.dump(yaml_dict, default_flow_style=True)
    return yaml.dump(yaml_dict, default_flow_style=True)


def main() -> None:
    '''Main function for the program.'''
    print("Loading JSON file...")
    json_dict = load_json_file()
    print("Converting JSON to YAML...")
    yaml_str = convert_json_to_yaml(json_dict)
    print("Writing YAML to file...")
    with open("scene.yaml", 'w', encoding="utf-8") as file:
        file.write(yaml_str)


if __name__ == '__main__':
    main()
