import React, { useState, useEffect } from 'react';
import { getKillSwitchStatus, enableTrading, disableTrading } from '../services/api';

interface KillSwitchStatus {
  isTradingEnabled: boolean;
}

const KillSwitch: React.FC = () => {
  const [status, setStatus] = useState<KillSwitchStatus | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    fetchStatus();
    const interval = setInterval(fetchStatus, 5000); // Check every 5 seconds
    return () => clearInterval(interval);
  }, []);

  const fetchStatus = async () => {
    try {
      const data = await getKillSwitchStatus();
      setStatus(data);
    } catch (error) {
      console.error('Failed to fetch kill switch status:', error);
    }
  };

  const toggleTrading = async (enable: boolean) => {
    setLoading(true);
    try {
      if (enable) {
        await enableTrading();
      } else {
        await disableTrading();
      }
      await fetchStatus(); // Refresh status
    } catch (error) {
      console.error('Failed to toggle trading:', error);
      alert('Failed to change trading status');
    } finally {
      setLoading(false);
    }
  };

  if (!status) {
    return <div>Loading kill switch status...</div>;
  }

  return (
    <div className="kill-switch">
      <h3>Emergency Kill Switch</h3>
      <div className="status-indicator">
        <span className={`status ${status.isTradingEnabled ? 'enabled' : 'disabled'}`}>
          Trading is {status.isTradingEnabled ? 'ENABLED' : 'DISABLED'}
        </span>
      </div>

      <div className="controls">
        <button
          className="enable-btn"
          onClick={() => toggleTrading(true)}
          disabled={status.isTradingEnabled || loading}
        >
          {loading ? 'Processing...' : 'Enable Trading'}
        </button>

        <button
          className="disable-btn"
          onClick={() => toggleTrading(false)}
          disabled={!status.isTradingEnabled || loading}
        >
          {loading ? 'Processing...' : 'EMERGENCY STOP'}
        </button>
      </div>

      <div className="warning">
        <p>⚠️ EMERGENCY STOP will immediately halt all automated trading activities.</p>
        <p>Use with caution - this is for emergency situations only.</p>
      </div>

      <style>
        {`
        .kill-switch {
          background: #fff;
          border: 2px solid #ddd;
          border-radius: 8px;
          padding: 20px;
          margin: 20px 0;
        }

        .status-indicator {
          margin-bottom: 15px;
        }

        .status {
          font-size: 18px;
          font-weight: bold;
          padding: 8px 16px;
          border-radius: 4px;
        }

        .status.enabled {
          background: #d4edda;
          color: #155724;
          border: 1px solid #c3e6cb;
        }

        .status.disabled {
          background: #f8d7da;
          color: #721c24;
          border: 1px solid #f5c6cb;
        }

        .controls {
          display: flex;
          gap: 10px;
          margin-bottom: 15px;
        }

        .controls button {
          padding: 12px 24px;
          border: none;
          border-radius: 4px;
          font-size: 16px;
          font-weight: bold;
          cursor: pointer;
          transition: background-color 0.2s;
        }

        .controls button:disabled {
          opacity: 0.6;
          cursor: not-allowed;
        }

        .enable-btn {
          background: #28a745;
          color: white;
        }

        .enable-btn:hover:not(:disabled) {
          background: #218838;
        }

        .disable-btn {
          background: #dc3545;
          color: white;
        }

        .disable-btn:hover:not(:disabled) {
          background: #c82333;
        }

        .warning {
          background: #fff3cd;
          border: 1px solid #ffeaa7;
          border-radius: 4px;
          padding: 12px;
          color: #856404;
        }

        .warning p {
          margin: 5px 0;
          font-size: 14px;
        }
        `}
      </style>
    </div>
  );
};

export default KillSwitch;