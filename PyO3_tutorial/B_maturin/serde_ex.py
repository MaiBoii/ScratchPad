import rust

from pydantic import BaseModel

class Human(BaseModel):
    name: str
    age: int

jan = Human(name="Jan", age=6)
print(jan.json())

rust.human_says_hi(jan.json())  # rust에게 json 전달