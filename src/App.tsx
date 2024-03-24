
import { AllComponents } from "./Components/AllComponents";
import NavBar from "./Components/NavBar";
function App() {
    return (
        <div className="min-w-screen min-h-screen flex flex-col justify-start items-start bg-gradient-to-b from-[#2a1640] to-black text-white gap-10 border border-white font-inter">
            <NavBar />
            <AllComponents />
        </div >
    );
}

export default App;
