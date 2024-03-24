import { useState } from "react";

function CustomResol() {
    const [selectedResolution, setSelectedResolution] = useState('');

    const handleResolutionChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        setSelectedResolution(event.target.value);
    };

    return (
        <div className="flex flex-col w-full h-full md:w-[250px] h-full md:h-full justify-start items-start gap-6 bg-zinc-950 border border-zinc-300 p-4 md:p-6">
            <div className="flex flex-col justify-start items-start gap-2">
                <h2 className="text-lg md:text-xl font-medium">Customize</h2>
                <h2 className="text-lg md:text-xl font-medium">Choose your resolution</h2>
            </div>
            <div className="flex flex-col justify-start items-start gap-3">
                <h2 className="text-lg font-medium">Resolution</h2>
                <form className="flex flex-col justify-center items-start gap-2">
                    <label htmlFor="resolution" className="text-white">Select a Resolution:</label>
                    <select
                        className="bg-red-500 text-white rounded-md p-2"
                        id="resolution"
                        value={selectedResolution}
                        onChange={handleResolutionChange}
                    >
                        <option value="" disabled>Select a resolution</option>
                        <option value="1920x1080">1920x1080 (Full HD)</option>
                        <option value="2560x1440">2560x1440 (Quad HD)</option>
                        <option value="3840x2160">3840x2160 (4K UHD)</option>
                        <option value="5120x2880">5120x2880 (5K)</option>
                    </select>
                </form>
            </div>
        </div>
    )
}

export default CustomResol;

