import json
import os
import xml.etree.ElementTree as ET


XML_DECLARATION = '<?xml version="1.0" encoding="UTF-8"?>\n'
DOCTYPE_DECLARATION = '<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">\n'
APPLICATION_DIR = '/Applications/'
app_list_dir = '../icon/app_list.json'
launch_daemon_file = 'com.ssuhung.appUpdateDetector.plist'
app_path = os.path.dirname(os.path.dirname(__file__)) + '/target/debug/app_icon'

with open(app_list_dir, "r") as json_file:
    app_list = json.load(json_file)
watch_paths_list = [os.path.join(APPLICATION_DIR, app_name + '.app') for app_name in app_list]

tree = ET.parse('./service_template.plist')
root = tree.getroot()
dict_ele = root.find('dict')

# Insert key ProgramArguments
key_ele = ET.Element('key')
key_ele.text = 'ProgramArguments'
dict_ele.append(key_ele)

# Insert ProgramArguments Array
paths_ele = ET.Element('array')
ele = ET.SubElement(paths_ele, 'string')
ele.text = app_path
dict_ele.append(paths_ele)

# Insert key WatchPaths
key_ele = ET.Element('key')
key_ele.text = 'WatchPaths'
dict_ele.append(key_ele)

# Insert WatchPaths Array
paths_ele = ET.Element('array')
for watch_path in watch_paths_list:
    ele = ET.SubElement(paths_ele, 'string')
    ele.text = watch_path
dict_ele.append(paths_ele)

ET.indent(tree)
# ET.dump(tree)

# Write the modified XML to file
print(f"Write modified XML file to { launch_daemon_file }")
if os.path.exists(launch_daemon_file):
    os.remove(launch_daemon_file)
with open(launch_daemon_file, 'w') as f:
    f.write(XML_DECLARATION)
    f.write(DOCTYPE_DECLARATION)
    f.write(ET.tostring(root, encoding='unicode'))
