import pandas as pd
import numpy as np
from sklearn.metrics import mean_absolute_error, mean_squared_error, r2_score
from scipy.stats import ks_2samp
import logging
import json
import os
from datetime import datetime

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class ModelMonitor:
    def __init__(self, model_name: str, reference_data_path: str, current_data_path: str):
        self.model_name = model_name
        self.reference_data_path = reference_data_path
        self.current_data_path = current_data_path

    def load_data(self, path: str) -> pd.DataFrame:
        if path.endswith('.csv'):
            return pd.read_csv(path)
        elif path.endswith('.json'):
            with open(path, 'r') as f:
                data = json.load(f)
            return pd.DataFrame(data)
        else:
            raise ValueError("Unsupported file format")

    def calculate_performance_metrics(self, y_true: np.ndarray, y_pred: np.ndarray) -> dict:
        mae = mean_absolute_error(y_true, y_pred)
        mse = mean_squared_error(y_true, y_pred)
        rmse = np.sqrt(mse)
        r2 = r2_score(y_true, y_pred)

        return {
            'mae': mae,
            'mse': mse,
            'rmse': rmse,
            'r2_score': r2
        }

    def detect_drift(self, reference_features: pd.DataFrame, current_features: pd.DataFrame) -> dict:
        drift_scores = {}
        for column in reference_features.columns:
            if column in current_features.columns:
                stat, p_value = ks_2samp(reference_features[column], current_features[column])
                drift_scores[column] = {
                    'ks_statistic': stat,
                    'p_value': p_value,
                    'drift_detected': p_value < 0.05  # Significance level
                }
        return drift_scores

    def monitor(self) -> dict:
        try:
            # Load reference and current data
            reference_data = self.load_data(self.reference_data_path)
            current_data = self.load_data(self.current_data_path)

            # Assume data has 'features' and 'target' columns
            reference_features = reference_data.drop(columns=['target'])
            reference_target = reference_data['target']
            current_features = current_data.drop(columns=['target'])
            current_target = current_data['target']

            # Simulate predictions (in real scenario, call the model API)
            # For demo, use random predictions
            np.random.seed(42)
            reference_predictions = reference_target + np.random.normal(0, 0.1, len(reference_target))
            current_predictions = current_target + np.random.normal(0, 0.1, len(current_target))

            # Calculate performance
            reference_metrics = self.calculate_performance_metrics(reference_target, reference_predictions)
            current_metrics = self.calculate_performance_metrics(current_target, current_predictions)

            # Detect drift
            drift_results = self.detect_drift(reference_features, current_features)

            # Check for performance degradation
            performance_degraded = (
                current_metrics['mae'] > reference_metrics['mae'] * 1.1 or  # 10% degradation
                current_metrics['r2_score'] < reference_metrics['r2_score'] * 0.9
            )

            result = {
                'model_name': self.model_name,
                'timestamp': datetime.utcnow().isoformat(),
                'reference_metrics': reference_metrics,
                'current_metrics': current_metrics,
                'drift_results': drift_results,
                'performance_degraded': performance_degraded,
                'alerts': []
            }

            if performance_degraded:
                result['alerts'].append("Performance degradation detected")

            drift_detected = any(d['drift_detected'] for d in drift_results.values())
            if drift_detected:
                result['alerts'].append("Data drift detected")

            logger.info(f"Monitoring completed for {self.model_name}")
            return result

        except Exception as e:
            logger.error(f"Error during monitoring: {str(e)}")
            return {'error': str(e)}

def main():
    # Example usage
    monitor = ModelMonitor(
        model_name="crypto_price_predictor",
        reference_data_path="data/reference_data.csv",  # Assume exists
        current_data_path="data/current_data.csv"      # Assume exists
    )

    results = monitor.monitor()
    print(json.dumps(results, indent=2))

    # In production, send to monitoring system or database

if __name__ == "__main__":
    main()