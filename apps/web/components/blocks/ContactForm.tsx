"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import { CheckIcon } from "@/components/ui/icons";
import { useAction } from "next-safe-action/hooks";
import { useForm } from "react-hook-form";
import * as z from "zod";

import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";

import { serverAction } from "@/actions/server-action";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Form } from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import { formSchema } from "@/lib/formSchema";

type Schema = z.infer<typeof formSchema>;

export function ContactForm() {
  const form = useForm<Schema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      email: "",
      company: "",
      employees: "",
      message: "",
      agree: false,
    } as unknown as Schema,
  });
  const formAction = useAction(serverAction, {
    onSuccess: () => {
      // TODO: show success message
      form.reset();
    },
    onError: () => {
      // TODO: show error message
    },
  });
  const handleSubmit = form.handleSubmit(async (data: Schema) => {
    formAction.execute(data);
  });

  const { isExecuting, hasSucceeded } = formAction;
  if (hasSucceeded) {
    return (
      <div className="w-full gap-2 rounded-md border p-2 sm:p-5 md:p-8">
        <div className="motion-safe:animate-slide-up h-full px-3 py-6">
          <div
            className="motion-safe:animate-scale-in mx-auto mb-4 flex w-fit justify-center rounded-full border p-2"
            style={{ "--scale-from": "0.5", animationDelay: "200ms" } as React.CSSProperties}
          >
            <CheckIcon className="size-8" />
          </div>
          <Heading level={2} className="mb-2 text-center text-pretty">
            Thank you
          </Heading>
          <Text color="muted" size="lg" className="text-center text-pretty">
            Form submitted successfully, we will get back to you soon
          </Text>
        </div>
      </div>
    );
  }

  return (
    <Form {...form}>
      <form
        onSubmit={handleSubmit}
        className="flex w-full flex-col gap-2 space-y-4 rounded-md"
      >
        <Form.Field
          control={form.control}
          name="name"
          rules={{ required: true }}
          render={({ field }) => (
            <Form.Item className="w-full">
              <Form.Label>Full name * </Form.Label>
              <Form.Control>
                <Input
                  type="text"
                  value={field.value}
                  onChange={(e) => {
                    const val = e.target.value;
                    field.onChange(val);
                  }}
                  placeholder="First and last name"
                />
              </Form.Control>

              <Form.Message />
            </Form.Item>
          )}
        />
        <Form.Field
          control={form.control}
          name="email"
          rules={{ required: true }}
          render={({ field }) => (
            <Form.Item className="w-full">
              <Form.Label>Email address * </Form.Label>
              <Form.Control>
                <Input
                  type="text"
                  value={field.value}
                  onChange={(e) => {
                    const val = e.target.value;
                    field.onChange(val);
                  }}
                  placeholder="me@company.com"
                />
              </Form.Control>

              <Form.Message />
            </Form.Item>
          )}
        />
        <Form.Field
          control={form.control}
          name="company"
          rules={{ required: false }}
          render={({ field }) => (
            <Form.Item className="w-full">
              <Form.Label>Company name </Form.Label>
              <Form.Control>
                <Input
                  type="text"
                  value={field.value}
                  onChange={(e) => {
                    const val = e.target.value;
                    field.onChange(val);
                  }}
                  placeholder="Company name"
                />
              </Form.Control>

              <Form.Message />
            </Form.Item>
          )}
        />

        <Form.Field
          control={form.control}
          rules={{ required: false }}
          name="employees"
          render={({ field }) => {
            const options = [
              { value: "1", label: "1" },
              { value: "2-10", label: "2-10" },
              { value: "11-50", label: "11-50" },
              { value: "51-500", label: "51-500" },
            ];
            return (
              <Form.Item className="w-full">
                <Form.Label>Number of employees </Form.Label>
                <Select onValueChange={field.onChange} value={field.value}>
                  <Form.Control>
                    <Select.Trigger className="w-full">
                      <Select.Value placeholder="e.g. 11-50" />
                    </Select.Trigger>
                  </Form.Control>
                  <Select.Content>
                    {options.map(({ label, value }) => (
                      <Select.Item key={value} value={value}>
                        {label}
                      </Select.Item>
                    ))}
                  </Select.Content>
                </Select>

                <Form.Message />
              </Form.Item>
            );
          }}
        />

        <Form.Field
          control={form.control}
          name="message"
          rules={{ required: true }}
          render={({ field }) => (
            <Form.Item>
              <Form.Label>Your message * </Form.Label>
              <Form.Control>
                <Textarea
                  {...field}
                  placeholder="Write your message"
                  className="resize-none"
                />
              </Form.Control>

              <Form.Message />
            </Form.Item>
          )}
        />
        <Form.Field
          control={form.control}
          rules={{ required: true }}
          name="agree"
          render={({ field }) => (
            <Form.Item className="flex flex-row items-start space-y-0 space-x-1">
              <Form.Control>
                <Checkbox
                  checked={field.value}
                  onCheckedChange={field.onChange}
                  required
                />
              </Form.Control>
              <div className="space-y-1 leading-none">
                <Form.Label>I agree to the terms and conditions</Form.Label>

                <Form.Message />
              </div>
            </Form.Item>
          )}
        />
        <div className="flex w-full items-center justify-end pt-3">
          <Button className="rounded-lg" size="sm">
            {isExecuting ? "Submitting..." : "Submit"}
          </Button>
        </div>
      </form>
    </Form>
  );
}
