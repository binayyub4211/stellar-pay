import { Hero } from "./components/hero";
import { Capabilities } from "./components/capabilities";
import { Architecture } from "./components/architecture";
import { Testimonials } from "./components/testimonials";
import { Security } from "./components/security";
import { Developer } from "./components/developer";
import { Footer } from "./components/footer";

export default function Home() {
  return (
    <div className="min-h-screen bg-black text-white antialiased">
      <Hero />
      <Developer />
      <Capabilities />
      <Architecture />
      <Testimonials />
      <Security />
      <Footer />
    </div>
  );
}