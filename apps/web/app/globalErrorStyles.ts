/* Minimal inline styles matching bnto theme tokens — no CSS file available. */

const sharedButtonStyle: React.CSSProperties = {
  padding: "0.5rem 1rem",
  borderRadius: "calc(1.25rem - 2px)",
  fontSize: "0.875rem",
  fontWeight: 500,
  fontFamily: "Inter, system-ui, sans-serif",
  cursor: "pointer",
  textDecoration: "none",
  display: "inline-flex",
  alignItems: "center",
  gap: "0.5rem",
  border: "none",
  transition: "opacity 0.15s ease-out",
};

export const bodyStyle: React.CSSProperties = {
  margin: 0,
  fontFamily: "Inter, system-ui, -apple-system, sans-serif",
  backgroundColor: "oklch(0.9899 0.0164 95.22)",
  color: "oklch(0.2628 0.0204 31.40)",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  minHeight: "100vh",
};

export const containerStyle: React.CSSProperties = {
  padding: "1.5rem",
  width: "100%",
  maxWidth: "32rem",
};

export const cardStyle: React.CSSProperties = {
  backgroundColor: "oklch(1.0000 0 0)",
  borderRadius: "1.25rem",
  padding: "2.5rem 2rem",
  textAlign: "center",
  boxShadow: "0 4px 6px -1px hsla(10, 20%, 15%, 0.08), 0 2px 4px -2px hsla(10, 20%, 15%, 0.06)",
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  gap: "1rem",
};

export const headingStyle: React.CSSProperties = {
  fontSize: "1.5rem",
  fontWeight: 600,
  fontFamily: "Geist, system-ui, sans-serif",
  margin: 0,
  letterSpacing: "0.02em",
};

export const textStyle: React.CSSProperties = {
  color: "oklch(0.5452 0.0251 31.20)",
  fontSize: "1rem",
  lineHeight: 1.6,
  margin: 0,
  textWrap: "balance",
};

export const errorMessageStyle: React.CSSProperties = {
  fontFamily: "Geist Mono, monospace",
  fontSize: "0.875rem",
  color: "oklch(0.5452 0.0251 31.20)",
  maxWidth: "100%",
  overflow: "hidden",
  textOverflow: "ellipsis",
  whiteSpace: "nowrap",
  padding: "0 1rem",
};

export const buttonRowStyle: React.CSSProperties = {
  display: "flex",
  gap: "0.5rem",
  paddingTop: "0.5rem",
  flexWrap: "wrap",
  justifyContent: "center",
};

export const primaryButtonStyle: React.CSSProperties = {
  ...sharedButtonStyle,
  backgroundColor: "oklch(0.6751 0.1788 35.19)",
  color: "white",
};

export const outlineButtonStyle: React.CSSProperties = {
  ...sharedButtonStyle,
  backgroundColor: "transparent",
  color: "oklch(0.2628 0.0204 31.40)",
  border: "1px solid oklch(0.8976 0.0168 95.25)",
};

export const ghostButtonStyle: React.CSSProperties = {
  ...sharedButtonStyle,
  backgroundColor: "transparent",
  color: "oklch(0.5452 0.0251 31.20)",
};
