import { BentoAuthProvider } from "@bento/auth/provider";
import { BentoCoreProvider } from "@bento/core/provider";
import { ThemeProvider } from "./theme-provider";

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <BentoAuthProvider>
      <BentoCoreProvider>
        <ThemeProvider>{children}</ThemeProvider>
      </BentoCoreProvider>
    </BentoAuthProvider>
  );
}
