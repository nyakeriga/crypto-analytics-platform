import React from 'react';
import Chart from '../components/Chart';
import SignalCard from '../components/SignalCard';
import IndicatorPanel from '../components/IndicatorPanel';

const Dashboard: React.FC = () => {
  return (
    <div>
      <h1>Dashboard</h1>
      <Chart />
      <SignalCard />
      <IndicatorPanel />
    </div>
  );
};

export default Dashboard;