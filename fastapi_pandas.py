import pandas as pd
import uvicorn
from fastapi import FastAPI

app = FastAPI()


@app.get("/")
async def root():
    df = pd.read_csv("data.csv")
    df2 = df.head(50_000)
    return df2.to_json(orient="records")

if __name__ == "__main__":
    uvicorn.run(app)
