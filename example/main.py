import meteorite
import json

app = meteorite.Meteorite()


@app.predict
def hello(data):
    body = bytearray(data).decode("utf-8")
    return {"key1": "value1", "key2": "value2"}


# This will run the hello() function
app.start()
