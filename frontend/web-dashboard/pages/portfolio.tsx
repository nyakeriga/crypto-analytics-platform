import React, { useState, useEffect } from 'react';
import { getPortfolio } from '../services/api';
import KillSwitch from '../components/KillSwitch';

interface PortfolioData {
  totalValue: number;
  totalPnl: number;
  totalPnlPercentage: number;
  positions: Position[];
  riskMetrics: RiskMetrics;
}

interface Position {
  symbol: string;
  quantity: number;
  avgPrice: number;
  currentPrice: number;
  pnl: number;
  pnlPercentage: number;
}

interface RiskMetrics {
  sharpeRatio: number;
  maxDrawdown: number;
  volatility: number;
  winRate: number;
}

const Portfolio: React.FC = () => {
  const [portfolioData, setPortfolioData] = useState<PortfolioData | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchPortfolioData();
    const interval = setInterval(fetchPortfolioData, 30000); // Update every 30 seconds
    return () => clearInterval(interval);
  }, []);

  const fetchPortfolioData = async () => {
    try {
      const data = await getPortfolio();
      setPortfolioData(data);
    } catch (error) {
      console.error('Failed to fetch portfolio data:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return <div>Loading portfolio data...</div>;
  }

  if (!portfolioData) {
    return <div>Failed to load portfolio data</div>;
  }

  return (
    <div className="portfolio-dashboard">
      <h1>Portfolio Management</h1>

      <KillSwitch />

      {/* Portfolio Overview */}
      <div className="portfolio-overview">
        <div className="metric-card">
          <h3>Total Value</h3>
          <p className="value">${portfolioData.totalValue.toFixed(2)}</p>
        </div>
        <div className="metric-card">
          <h3>Total P&L</h3>
          <p className={`value ${portfolioData.totalPnl >= 0 ? 'positive' : 'negative'}`}>
            ${portfolioData.totalPnl.toFixed(2)} ({portfolioData.totalPnlPercentage.toFixed(2)}%)
          </p>
        </div>
      </div>

      {/* Risk Metrics */}
      <div className="risk-metrics">
        <h2>Risk Metrics</h2>
        <div className="metrics-grid">
          <div className="metric">
            <span>Sharpe Ratio:</span>
            <span>{portfolioData.riskMetrics.sharpeRatio.toFixed(2)}</span>
          </div>
          <div className="metric">
            <span>Max Drawdown:</span>
            <span>{(portfolioData.riskMetrics.maxDrawdown * 100).toFixed(2)}%</span>
          </div>
          <div className="metric">
            <span>Volatility:</span>
            <span>{(portfolioData.riskMetrics.volatility * 100).toFixed(2)}%</span>
          </div>
          <div className="metric">
            <span>Win Rate:</span>
            <span>{(portfolioData.riskMetrics.winRate * 100).toFixed(2)}%</span>
          </div>
        </div>
      </div>

      {/* Positions Table */}
      <div className="positions-table">
        <h2>Open Positions</h2>
        <table>
          <thead>
            <tr>
              <th>Symbol</th>
              <th>Quantity</th>
              <th>Avg Price</th>
              <th>Current Price</th>
              <th>P&L</th>
              <th>P&L %</th>
            </tr>
          </thead>
          <tbody>
            {portfolioData.positions.map((position) => (
              <tr key={position.symbol}>
                <td>{position.symbol}</td>
                <td>{position.quantity}</td>
                <td>${position.avgPrice.toFixed(2)}</td>
                <td>${position.currentPrice.toFixed(2)}</td>
                <td className={position.pnl >= 0 ? 'positive' : 'negative'}>
                  ${position.pnl.toFixed(2)}
                </td>
                <td className={position.pnlPercentage >= 0 ? 'positive' : 'negative'}>
                  {position.pnlPercentage.toFixed(2)}%
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <style>
        {`
        .portfolio-dashboard {
          padding: 20px;
        }
        .portfolio-overview {
          display: flex;
          gap: 20px;
          margin-bottom: 30px;
        }
        .metric-card {
          background: #f5f5f5;
          padding: 20px;
          border-radius: 8px;
          flex: 1;
        }
        .metric-card h3 {
          margin: 0 0 10px 0;
          color: #666;
        }
        .value {
          font-size: 24px;
          font-weight: bold;
          margin: 0;
        }
        .positive {
          color: #28a745;
        }
        .negative {
          color: #dc3545;
        }
        .risk-metrics {
          margin-bottom: 30px;
        }
        .metrics-grid {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
          gap: 15px;
        }
        .metric {
          display: flex;
          justify-content: space-between;
          padding: 10px;
          background: #f8f9fa;
          border-radius: 4px;
        }
        .positions-table table {
          width: 100%;
          border-collapse: collapse;
        }
        .positions-table th,
        .positions-table td {
          padding: 12px;
          text-align: left;
          border-bottom: 1px solid #ddd;
        }
        .positions-table th {
          background: #f8f9fa;
          font-weight: 600;
        }
      `}</style>
    </div>
  );
};

export default Portfolio;