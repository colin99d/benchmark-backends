import os
import csv
import random
import string
from tqdm import tqdm
import requests


def get_random_string(length):
    # choose from all lowercase letter
    letters = string.ascii_lowercase
    return "".join(random.choice(letters) for i in range(length))


def get_avg_time(times: int = 10):
    all_times: list[float] = []
    for _ in tqdm(range(times)):
        response = requests.get("http://localhost:8000/")
        time = response.elapsed.total_seconds()
        all_times.append(time)
    return sum(all_times) / len(all_times)


def main():
    print("Benchmarking backends\n")

    file_name = "data.csv"
    if not os.path.isfile(file_name):
        print("Generating 1,000,000 rows and 20 columns into a csv")
        with open(file_name, mode="w") as file:
            file_writer = csv.writer(
                file, delimiter=",", quotechar='"', quoting=csv.QUOTE_MINIMAL
            )
            file_writer.writerow([f"{i}_name" for i in range(20)])

            for _ in tqdm(range(1_000_000)):
                file_writer.writerow([get_random_string(20) for _ in range(20)])

    print("and then serializing and returning the first 50,000 rows.")

    print("\nRocket and polars")
    input("Run the rocket-polars server and then press enter once it is set up")
    rocket_polars = get_avg_time(10)
    print(f"Average time: {rocket_polars}")

    print("\nFastApi and Pandas")
    input("Run python fastapi_pandas.py and then press enter once it is set up")
    fastapi_pandas = get_avg_time(10)
    print(f"Average time: {fastapi_pandas}")


if __name__ == "__main__":
    main()
