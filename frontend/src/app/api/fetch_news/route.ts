import axios from 'axios';
import type { NextApiRequest, NextApiResponse } from 'next';

type Article = {
  source: {
    id: string | null;
    name: string;
  };
  author: string | null;
  title: string;
  description: string | null;
  url: string;
  urlToImage: string | null;
  publishedAt: string;
  content: string | null;
};

type NewsResponse = {
  status: string;
  totalResults: number;
  articles: Article[];
};

type ErrorResponse = {
  message: string;
};

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<NewsResponse | ErrorResponse>
) {
  try {
    const response = await axios.get<NewsResponse>('http://localhost:3030');
    res.status(200).json(response.data);
  } catch (error) {
    res.status(500).json({ message: 'Failed to fetch news' });
  }
}
