import {
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogClose,
  DialogBody,
  DialogFooter,
  Button,
  Row,
} from "@bnto/ui";

interface AuthGateDialogBodyProps {
  title: string;
  description: string;
}

export function AuthGateDialogBody({ title, description }: AuthGateDialogBodyProps) {
  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>{title}</DialogTitle>
        <DialogClose />
      </DialogHeader>
      <DialogBody>
        <DialogDescription>{description}</DialogDescription>
      </DialogBody>
      <DialogFooter>
        <Row className="gap-3 pt-2">
          <Button href="/signin" variant="primary" elevation="sm">
            Sign up free
          </Button>
          <Button href="/signin" variant="outline">
            Sign in
          </Button>
        </Row>
      </DialogFooter>
    </DialogContent>
  );
}
