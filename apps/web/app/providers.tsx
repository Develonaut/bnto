import { BntoAuthProvider } from "@bnto/auth/provider";
import { BntoCoreProvider } from "@bnto/core/provider";
import { ThemeProvider } from "./theme-provider";

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <BntoAuthProvider>
      <BntoCoreProvider>
        <ThemeProvider>{children}</ThemeProvider>
      </BntoCoreProvider>
    </BntoAuthProvider>
  );
}
