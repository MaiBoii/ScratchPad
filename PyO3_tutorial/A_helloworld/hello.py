from ctypes import cdll

lib = cdll.LoadLibrary("./target/release/libhelloworld.so")
lib.hello()
print("헬로월드!!")