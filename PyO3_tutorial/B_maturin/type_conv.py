import rust

sum = rust.list_sum([10, 10, 10, 10, 10])
print(sum)

a_dict = {
    "key 1": "value 1",
    "key 2": "value 2",
    "key 3": "value 3",
    "key 4": "value 4",
    }
rust.dict_printer(a_dict)

"""
try:
    rust.dict_printer({"a": 1, "b": 2}) # 일부러
except TypeError as e:
    print("Type Error: {e}")
"""

rust.word_printer("hello", 3, False, True)
rust.word_printer("eyb", 2, True, False)