import GenerateProof from "./components/GenerateProof/generateProof.jsx";
import VerifyProof from "./components/VerifyProof/verifyProof.jsx";
import { Routes, Route } from 'react-router-dom'

function App() {
  return (
    <Routes>
      <Route path="/" element={<GenerateProof />} />
      <Route path="/verify" element={<VerifyProof />} />
    </Routes>
  );
}

export default App;
