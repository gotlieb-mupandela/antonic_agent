import { splitProps } from "solid-js";
import type { JSX } from "solid-js";

type ButtonProps = JSX.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "primary" | "secondary" | "ghost" | "outline" | "danger";
};

export default function Button(props: ButtonProps) {
  const [local, rest] = splitProps(props, ["variant", "class", "disabled", "title", "type"]);
  const variant = () => local.variant ?? "primary";

  const base =
    "inline-flex items-center justify-center gap-2 rounded-xl px-4 py-2 text-sm font-medium transition-all duration-150 active:scale-[0.98] focus:outline-none focus:ring-2 focus:ring-[rgba(var(--dls-accent-rgb),0.3)] disabled:opacity-50 disabled:cursor-not-allowed";

  const variants: Record<NonNullable<ButtonProps["variant"]>, string> = {
    primary: "bg-dls-accent text-white hover:bg-[var(--dls-accent-hover)] border border-transparent shadow-lg shadow-[rgba(var(--dls-accent-rgb),0.15)]",
    secondary: "bg-gray-12 text-gray-1 hover:bg-gray-11 border border-transparent font-semibold shadow-md",
    ghost: "bg-transparent text-dls-secondary hover:text-dls-text hover:bg-dls-hover",
    outline: "border border-dls-border text-dls-text hover:bg-dls-hover hover:border-dls-accent/50 bg-transparent",
    danger: "bg-red-3 text-red-11 hover:bg-red-4 border border-red-6",
  };

  return (
    <button
      {...rest}
      type={local.type ?? "button"}
      disabled={local.disabled}
      aria-disabled={local.disabled}
      title={local.title}
      class={`${base} ${variants[variant()]} ${local.class ?? ""}`.trim()}
    />
  );
}
