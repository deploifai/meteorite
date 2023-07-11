import meteorite

app = meteorite.Meteorite()


@app.predict
def hello(data):
    body = data.decode("utf-8")
    return body


app.start()
