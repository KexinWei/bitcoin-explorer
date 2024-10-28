import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './App.css';
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend,
} from 'chart.js';

ChartJS.register(
  CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend
);

interface MarketDataPoint {
  timestamp: string;
  price_usd: number;
  volume_usd: number;
}

interface NetworkDataPoint {
  timestamp: string;
  hash_rate: number;
  difficulty: number;
}

interface LatestBlock {
  block_hash: string;
  height: number;
  timestamp: string;
  tx_count: number;
  size: number;
  weight: number;
}

const App: React.FC = () => {
  const [latestBlock, setLatestBlock] = useState<LatestBlock | null>(null);
  const [marketData, setMarketData] = useState<MarketDataPoint[]>([]);
  const [networkData, setNetworkData] = useState<NetworkDataPoint[]>([]);
  const [bitcoinPrice, setBitcoinPrice] = useState<number | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [blockResponse, marketResponse, networkResponse] = await Promise.all([
          axios.get('http://localhost:3001/latest-block'),
          axios.get('http://localhost:3001/market-data'),
          axios.get('http://localhost:3001/network-data'),
        ]);

        setLatestBlock(blockResponse.data);
        setMarketData(marketResponse.data);
        setNetworkData(networkResponse.data);

        if (marketResponse.data.length > 0) {
          const latestMarketPoint = marketResponse.data[marketResponse.data.length - 1];
          setBitcoinPrice(latestMarketPoint.price_usd);
        }

        setLoading(false);
      } catch (error) {
        console.error("Error fetching data:", error);
        setLoading(false);
      }
    };

    fetchData();

    const interval = setInterval(() => {
      fetchData();
    }, 10000);

    return () => clearInterval(interval);
  }, []);

  const hashRateDifficultyChartData = {
    labels: networkData.map(point => point.timestamp),
    datasets: [
      {
        label: 'Hash Rate',
        data: networkData.map(point => point.hash_rate),
        borderColor: '#00ffea',
        backgroundColor: 'rgba(0, 255, 234, 0.2)',
        yAxisID: 'y',
      },
      {
        label: 'Difficulty',
        data: networkData.map(point => point.difficulty),
        borderColor: '#ff00ff',
        backgroundColor: 'rgba(255, 0, 255, 0.2)',
        yAxisID: 'y1',
      },
    ],
  };

  const priceVolumeChartData = {
    labels: marketData.map(point => point.timestamp),
    datasets: [
      {
        label: 'Price (USD)',
        data: marketData.map(point => point.price_usd),
        borderColor: '#00ffea',
        backgroundColor: 'rgba(0, 255, 234, 0.2)',
        yAxisID: 'y',
      },
      {
        label: 'Volume (USD)',
        data: marketData.map(point => point.volume_usd),
        borderColor: '#ff00ff',
        backgroundColor: 'rgba(255, 0, 255, 0.2)',
        yAxisID: 'y1',
      },
    ],
  };

  const commonChartOptions = {
    responsive: true,
    interaction: {
      mode: 'index' as const,
      intersect: false,
    },
    stacked: false,
    plugins: {
      legend: {
        labels: {
          color: '#00ffea',
        },
      },
      title: {
        display: false,
      },
    },
    scales: {
      x: {
        ticks: {
          color: '#00ffea',
        },
        grid: {
          color: '#333',
        },
      },
      y: {
        type: 'linear' as const,
        display: true,
        position: 'left' as const,
        ticks: {
          color: '#00ffea',
        },
        grid: {
          color: '#333',
        },
      },
      y1: {
        type: 'linear' as const,
        display: true,
        position: 'right' as const,
        ticks: {
          color: '#ff00ff',
        },
        grid: {
          drawOnChartArea: false,
          color: '#333',
        },
      },
    },
  };

  const hashRateDifficultyChartOptions = {
    ...commonChartOptions,
  };

  const priceVolumeChartOptions = {
    ...commonChartOptions,
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Bitcoin Explorer</h1>
      </header>
      <div className="content">
        {loading ? (
          <p>Loading data...</p>
        ) : (
          <>
            <div className="left-side">
              <div className="data-box">
                <h2>Latest Block Information</h2>
                <p><strong>Block Height:</strong> {latestBlock?.height}</p>
                <p><strong>Block Hash:</strong> {latestBlock?.block_hash}</p>
                <p><strong>Timestamp:</strong> {latestBlock?.timestamp}</p>
                <p><strong>Transaction Count:</strong> {latestBlock?.tx_count}</p>
                <p><strong>Size:</strong> {latestBlock?.size} bytes</p>
                <p><strong>Weight:</strong> {latestBlock?.weight}</p>
              </div>
              <div className="chart-container">
                <h2>Hash Rate and Difficulty Trends</h2>
                <Line data={hashRateDifficultyChartData} options={hashRateDifficultyChartOptions} />
              </div>
            </div>
            <div className="right-side">
              <div className="data-box">
                <h2>Current Bitcoin Price</h2>
                <p className="price">${bitcoinPrice?.toLocaleString()}</p>
              </div>
              <div className="chart-container">
                <h2>Price and Volume Trends</h2>
                <Line data={priceVolumeChartData} options={priceVolumeChartOptions} />
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default App;
