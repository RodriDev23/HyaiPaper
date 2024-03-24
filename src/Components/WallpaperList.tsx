import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { convertFileSrc } from '@tauri-apps/api/tauri';

export const WallpaperList: React.FC = () => {
    const [imageUrls, setImageUrls] = useState<string[]>([]);
    const [selectedImageIndex, setSelectedImageIndex] = useState<number | null>(null);

    // we fetch the pictures and we convert it into an array
    useEffect(() => {
        const fetchDataAndSendToBackend = async () => {
            try {
                const response = await invoke<string[]>('send_wallpapers_user');
                if (response && response.length > 0) {
                    const convertedUrls = response.map(filePath => convertFileSrc(filePath));
                    setImageUrls(convertedUrls);
                    if (selectedImageIndex !== null) {
                        const imageName = convertedUrls[selectedImageIndex];
                        console.log("Name of selected image:", imageName);
                        await invoke('get_img_address', { imgAddress: imageName });
                    }
                } else {
                    console.error('Empty response received from server.');
                }
            } catch (error) {
                console.error('Error fetching image URLs:', error);
            }
        };
        fetchDataAndSendToBackend();
    }, [selectedImageIndex]);

    const getImageSelectHandler = (index: number) => {
        setSelectedImageIndex(index);
    };

    return (
        <div className="w-full md:w-full h-full flex flex-col justify-center items-center gap-20 pl-10 pb-10">
            <h2 className="text-3xl text-start md:text-4xl text-white font-bold self-start ml-5">Wallpaper List</h2>
            <div className="grid w-full grid-cols-2 gap-5">
                {imageUrls.map((imageUrl, index) => (
                    <div key={index} className="flex flex-col justify-center items-center gap-y-5 mt-3">
                        <img
                            className="aspect-video w-[90%] object-cover rounded-lg hover:cursor-pointer hover:border hover:border-white transition-transform duration-300 ease-in-out transform hover:-translate-y-2 hover:scale-105"
                            src={imageUrl}
                            alt="Wallpaper"
                            onClick={() => getImageSelectHandler(index)}
                        />
                    </div>
                ))}
            </div>
        </div>
    );
};

