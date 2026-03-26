import { PlusIcon } from "../../icons";
import { Button } from "../Button";

interface KeyValueAddButtonProps {
  onClick: () => void;
}

export function KeyValueAddButton({ onClick }: KeyValueAddButtonProps) {
  return (
    <Button type="button" variant="outline" onClick={onClick} className="self-start">
      <PlusIcon />
      Add pair
    </Button>
  );
}
