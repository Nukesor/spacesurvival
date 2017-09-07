import yaml

with open("example.yaml", 'r') as stream:
    try:
        print(yaml.load(stream))
