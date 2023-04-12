import json
from meteorite import Meteorite

app = Meteorite()

@app.predict
def main(data):
  data = json.loads(data)
  print(data["key"])
  return data


app.start(port=5001)
