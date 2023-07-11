import typing


class Meteorite:
    """
    A fast and simple server app for machine learning models.
    """

    def __init__(self) -> None:
        """
        Initializes the Meteorite server app
        """

    def predict(self, wraps: typing.Callable[[bytes], typing.Union[str, dict[str, any]]]) -> None:
        """
        Decorator to wrap a model inference function.
        The inference function should take in a bytes object and return a string or a dictionary.
        The inference endpoint will be available at `/predict`.

        :param wraps: model inference function

        >>> app = Meteorite()
        >>> @app.predict
        >>> def infer(data: bytes) -> str:
        >>>     return "Hello World"
        """

    def start(self, port: int = 4000) -> None:
        """
        Starts the server app

        :param port: port to run the server on, defaults to 4000
        """
