import { Background } from "@/components/Background";
import { FAQ } from "@/components/blocks/FAQ";

const Page = () => {
  return (
    <Background>
      <FAQ
        className="py-28 text-center lg:pt-44 lg:pb-32"
        className2="max-w-xl lg:grid-cols-1"
        headerTag="h1"
      />
    </Background>
  );
};

export default Page;
