import type { JSX } from "solid-js";

type Props = {
  size?: number;
  class?: string;
};

export default function AntonicAgentLogo(props: Props): JSX.Element {
  const size = props.size ?? 24;
  return (
    <img
      src="/openwork-logo.svg"
      alt="Antonic Agent"
      width={size}
      height={size}
      class={`inline-block ${props.class ?? ""}`}
    />
  );
}
