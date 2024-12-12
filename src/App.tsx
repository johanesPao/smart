import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { motion } from "motion/react";

const App = () => {
  const [welcomeMessage, setWelcomeMessage] = useState("");

  useEffect(() => {
    const fetchWelcomeMessage = async () => {
      const message = await invoke<string>("welcome");
      if (message) {
        setWelcomeMessage(message);
      }
    }
    fetchWelcomeMessage();
  }, []);

  return (
    <motion.div 
      className="w-fit mx-auto text-center"
      animate={{ x: [200, 0], opacity: [0, 1] }}
      transition={{ duration: 2.25, ease: "easeInOut" }}
    >
      <h1 className="overflow-hidden">SOSCO Merchandising Analytics and Reporting Tool</h1>
      {welcomeMessage && <p>{welcomeMessage}</p>}
    </motion.div>
  );
}

export default App;
