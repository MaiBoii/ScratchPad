import timeit

def get_fibonacci(number: int) -> int:
    """Get the nth Fibonacci number."""
    if number == 1:
        return 1
    elif number == 2:
        return 2

    total = 0
    last = 0
    current = 1
    for _ in range(1, number):
        total = last + current
        last = current
        current = total
    return total


print("python fib(5): ", timeit.timeit("get_fibonacci(5)", setup='from __main__ import get_fibonacci'))
print("rust fib(5): ", timeit.timeit("get_fibonacci(5)", setup="from rust import get_fibonacci"))

print("rust fib(150): ", timeit.timeit("get_fibonacci(150)", setup='from __main__ import get_fibonacci'))
print("rust fib(150): ", timeit.timeit("get_fibonacci(150)", setup="from rust import get_fibonacci"))