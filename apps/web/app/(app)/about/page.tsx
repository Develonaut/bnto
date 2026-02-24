import { Background } from "#components/Background";
import About from "#components/blocks/About";
import { AboutHero } from "#components/blocks/AboutHero";
import { Investors } from "#components/blocks/Investors";
import { DashedLine } from "#components/DashedLine";

export default function AboutPage() {
  return (
    <Background>
      <div className="py-28 lg:py-32 lg:pt-44">
        <AboutHero />

        <About />
        <div className="pt-28 lg:pt-32">
          <DashedLine className="container max-w-5xl scale-x-115" />
          <Investors />
        </div>
      </div>
    </Background>
  );
}
