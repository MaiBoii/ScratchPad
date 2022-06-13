import rust
import logging

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)

logging.info("Logging from the Python code")
rust.log_example()

print()
rust.log_different_levels()