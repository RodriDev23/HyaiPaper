import { IoSunnySharp } from "react-icons/io5";

function NavBar() {
    return (
        <header className="py-4 w-full">
            <div className="flex items-center justify-between max-w-full px-3 w-full">
                {/* Left side */}
                <div className="flex items-center gap-2">
                    <a href="#" className="font-semibold flex items-center gap-2">
                        <IoSunnySharp className="w-6 h-6 fill-primary" />
                        <span className="font-semibold text-2xl">HyaiPaper</span>
                    </a>
                </div>
                {/* Right side */}
                <div>
                    <nav className="md:flex items-center gap-6">
                        <ul className="flex items-center gap-6">
                            <li>
                                <a href="#" className="font-medium border-b-2 border-primary dark:border-primary">Home</a>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
        </header>
    );
}

export default NavBar;

