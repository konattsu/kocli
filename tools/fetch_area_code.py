import json
import re
import requests


def area_code(res_json):
    PATTERN_CODE = r"....00"
    FILE_PATH = "./area_code.json"

    area_code_list: list = []
    area_detail_code_list: list = []

    for office in res_json["offices"].items():
        if re.match(PATTERN_CODE, office[0]):
            area_code_list.append(office[0].replace("'", "\""))
            for child in office[1]["children"]:
                area_detail_code_list.append(child.replace("'", "\""))

    output = {"code": area_detail_code_list}
    create_file(output, FILE_PATH)


def create_file(dict_contents: dict, path: str):
    with open(path, mode="w") as f:
        json.dump(dict_contents, f, indent=4)

    # 最後に改行を無理やりつける
    tmp: str = ""
    with open(path, "r") as f:
        tmp = f.read()
    with open(path, "w") as f:
        f.write(f"{tmp}\n")
    print("create file finished")


URL = "https://www.jma.go.jp/bosai/common/const/area.json"

res_json = requests.get(URL).json()
area_code(res_json)
print("ファイル内容いい感じやったら ./resources/ に移動")
