import axios from 'axios';
import Image from 'next/image';
import Noimage from './no-image-icon-23494.png';

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

async function fetchNews(): Promise<Article[]> {
  try {
    const res = await axios.get<Article[]>('http://localhost:3030');
    return res.data;
  } catch (error) {
    console.error("Failed to fetch news", error);
    return [];
  }
}

const NewsPage = async () => {
  const news = await fetchNews();

  return (
    <div>
      <div>
        <h1 className="text-center py-4 text-4xl font-black">NEWS</h1>
      </div>
      <div className="max-w-[1320px] mx-auto grid sm:grid-cols-1 lg:grid-cols-4 md:grid-cols-2 gap-6 px-[20px]">
        {news.map((item) => (
          <div key={item.url} className="bg-white shadow-2xl rounded-2xl hover:shadow-2xl transition duration-300 ease-in-out transform hover:-translate-y-6">
            <div className="mt-4 cursor-pointer">
              {item.urlToImage ? (
                <img className="h-60 object-cover w-full" src={item.urlToImage} alt="news image" width={500} height={300} />
              ) : (
                <Image alt="No image" src={Noimage} width={500} height={300} />
              )}
            </div>
            <div className="p-4">
              <p className="text-slate-500">{item.title}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default NewsPage;
