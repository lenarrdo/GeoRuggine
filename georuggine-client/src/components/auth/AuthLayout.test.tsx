import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { AuthLayout } from "./AuthLayout";

describe("AuthLayout", () => {
  it("renders the brand and the provided children", () => {
    render(
      <AuthLayout>
        <div>auth form</div>
      </AuthLayout>,
    );

    expect(screen.getByText("GeoRust")).toBeInTheDocument();
    expect(screen.getByText("Track. Communicate. Analyze.")).toBeInTheDocument();
    expect(screen.getByText("auth form")).toBeInTheDocument();
  });
});
