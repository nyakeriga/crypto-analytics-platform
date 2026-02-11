const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL || 'http://localhost:3001';

interface Signal {
  id: string;
  symbol: string;
  type: 'buy' | 'sell';
  price: number;
  timestamp: string;
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

interface PortfolioData {
  totalValue: number;
  totalPnl: number;
  totalPnlPercentage: number;
  positions: Position[];
  riskMetrics: RiskMetrics;
}

export const fetchSignals = async (): Promise<Signal[]> => {
  const response = await fetch(`${API_BASE_URL}/signals`);
  if (!response.ok) {
    throw new Error('Failed to fetch signals');
  }
  return response.json();
};

export const fetchChartData = async (symbol: string): Promise<any> => {
  const response = await fetch(`${API_BASE_URL}/chart/${symbol}`);
  if (!response.ok) {
    throw new Error('Failed to fetch chart data');
  }
  return response.json();
};

export const getPortfolio = async (): Promise<PortfolioData> => {
  const response = await fetch(`${API_BASE_URL}/portfolio`);
  if (!response.ok) {
    throw new Error('Failed to fetch portfolio data');
  }
  return response.json();
};

export const getKillSwitchStatus = async (): Promise<{ isTradingEnabled: boolean }> => {
  const response = await fetch(`${API_BASE_URL}/killswitch/status`);
  if (!response.ok) {
    throw new Error('Failed to fetch kill switch status');
  }
  return response.json();
};

export const enableTrading = async (): Promise<void> => {
  const response = await fetch(`${API_BASE_URL}/killswitch/enable`, { method: 'POST' });
  if (!response.ok) {
    throw new Error('Failed to enable trading');
  }
};

export const disableTrading = async (): Promise<void> => {
  const response = await fetch(`${API_BASE_URL}/killswitch/disable`, { method: 'POST' });
  if (!response.ok) {
    throw new Error('Failed to disable trading');
  }
};