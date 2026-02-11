from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import joblib
import numpy as np
import logging
from prometheus_client import Counter, Histogram, generate_latest, CONTENT_TYPE_LATEST
import uvicorn

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app = FastAPI(title="Crypto Model Serving API", version="1.0.0")

# Prometheus metrics
inference_counter = Counter('model_inference_total', 'Total number of inferences', ['model_name'])
inference_duration = Histogram('model_inference_duration_seconds', 'Inference duration in seconds', ['model_name'])

class PredictionRequest(BaseModel):
    features: list[float]
    model_name: str = "default_model"

class PredictionResponse(BaseModel):
    prediction: float
    model_version: str = "v1.0"

# Load model (in production, load from a model registry or file)
MODEL_PATH = "models/default_model.pkl"

try:
    model = joblib.load(MODEL_PATH)
    logger.info(f"Model loaded from {MODEL_PATH}")
except FileNotFoundError:
    logger.warning(f"Model file {MODEL_PATH} not found. Using dummy model.")
    from sklearn.linear_model import LinearRegression
    model = LinearRegression()
    # Dummy training
    X = np.array([[1, 2], [3, 4], [5, 6]])
    y = np.array([1.5, 3.5, 5.5])
    model.fit(X, y)

@app.get("/health")
async def health_check():
    return {"status": "healthy"}

@app.get("/metrics")
async def metrics():
    return generate_latest(), {"Content-Type": CONTENT_TYPE_LATEST}

@app.post("/predict", response_model=PredictionResponse)
async def predict(request: PredictionRequest):
    try:
        with inference_duration.labels(model_name=request.model_name).time():
            features = np.array(request.features).reshape(1, -1)
            prediction = model.predict(features)[0]
            inference_counter.labels(model_name=request.model_name).inc()
            logger.info(f"Prediction made: {prediction} for model {request.model_name}")
            return PredictionResponse(prediction=float(prediction))
    except Exception as e:
        logger.error(f"Error during prediction: {str(e)}")
        raise HTTPException(status_code=500, detail=f"Prediction failed: {str(e)}")

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)