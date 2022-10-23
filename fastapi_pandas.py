from datetime import datetime
import pandas as pd
import uvicorn
from fastapi import FastAPI

app = FastAPI()


@app.get("/")
async def root():
    now = datetime.now()
    df = pd.read_csv("data.csv")
    loading = datetime.now()
    df2 = df.head(50_000)
    slicing = datetime.now()
    json = df2.to_json(orient="records")
    converting = datetime.now()
    l_time = (loading - now).total_seconds()
    s_time = (slicing - loading).total_seconds()
    c_time = (converting - slicing).total_seconds()
    print(f"Loading: {l_time}, Slicing: {s_time}, Converting: {c_time}")
    return json


if __name__ == "__main__":
    uvicorn.run(app)
