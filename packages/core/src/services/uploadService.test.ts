import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

// Mock the adapter before importing the service
vi.mock("../adapters/convex/uploadAdapter", () => ({
  generateUploadUrls: vi.fn(),
}));

import { createUploadService } from "./uploadService";
import { generateUploadUrls } from "../adapters/convex/uploadAdapter";
import type { UploadSession, FileUploadProgress } from "../types/upload";

const mockGenerateUploadUrls = vi.mocked(generateUploadUrls);

// Stub XMLHttpRequest for upload tests.
// Returns the shared instance so tests can control status and trigger events.
function installMockXhr() {
  const uploadListeners: Record<string, EventListener> = {};
  const xhrListeners: Record<string, EventListener> = {};

  const instance = {
    open: vi.fn(),
    setRequestHeader: vi.fn(),
    send: vi.fn(),
    status: 200,
    upload: {
      addEventListener: vi.fn(
        (event: string, handler: EventListener) => {
          uploadListeners[event] = handler;
        },
      ),
    },
    addEventListener: vi.fn(
      (event: string, handler: EventListener) => {
        xhrListeners[event] = handler;
      },
    ),
  };

  // The constructor must return `instance` so status changes are visible
  // to the code under test (uploadFileToR2 checks `xhr.status`).
  vi.stubGlobal(
    "XMLHttpRequest",
    function MockXHR() {
      return instance;
    },
  );

  return {
    instance,
    fireUploadProgress: (loaded: number, total: number) => {
      uploadListeners.progress?.({
        lengthComputable: true,
        loaded,
        total,
      } as unknown as Event);
    },
    fireLoad: () => xhrListeners.load?.({} as Event),
    fireError: () => xhrListeners.error?.({} as Event),
  };
}

describe("createUploadService", () => {
  let service: ReturnType<typeof createUploadService>;

  beforeEach(() => {
    vi.clearAllMocks();
    service = createUploadService();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  describe("generateUrls", () => {
    it("delegates to the adapter", async () => {
      const session: UploadSession = {
        sessionId: "test-session",
        urls: [{ name: "file.png", key: "uploads/test/file.png", url: "https://r2.example.com/file.png" }],
        expiresAt: Date.now() + 900_000,
      };
      mockGenerateUploadUrls.mockResolvedValueOnce(session);

      const result = await service.generateUrls([
        { name: "file.png", contentType: "image/png", sizeBytes: 1024 },
      ]);

      expect(result).toEqual(session);
      expect(mockGenerateUploadUrls).toHaveBeenCalledWith([
        { name: "file.png", contentType: "image/png", sizeBytes: 1024 },
      ]);
    });
  });

  describe("uploadFiles", () => {
    it("generates presigned URLs from file metadata", async () => {
      const session: UploadSession = {
        sessionId: "session-123",
        urls: [
          { name: "a.png", key: "uploads/session-123/a.png", url: "https://r2/a" },
        ],
        expiresAt: Date.now() + 900_000,
      };
      mockGenerateUploadUrls.mockResolvedValueOnce(session);

      const { instance, fireLoad } = installMockXhr();

      const file = new File(["data"], "a.png", { type: "image/png" });

      instance.send.mockImplementation(() => {
        instance.status = 200;
        fireLoad();
      });

      const result = await service.uploadFiles([file]);

      expect(result.sessionId).toBe("session-123");
      expect(mockGenerateUploadUrls).toHaveBeenCalledWith([
        { name: "a.png", contentType: "image/png", sizeBytes: 4 },
      ]);
    });

    it("sends PUT request to presigned URL with correct content type", async () => {
      const session: UploadSession = {
        sessionId: "session-put",
        urls: [
          { name: "img.jpg", key: "uploads/session-put/img.jpg", url: "https://r2/img" },
        ],
        expiresAt: Date.now() + 900_000,
      };
      mockGenerateUploadUrls.mockResolvedValueOnce(session);

      const { instance, fireLoad } = installMockXhr();
      instance.send.mockImplementation(() => {
        instance.status = 200;
        fireLoad();
      });

      const file = new File(["jpeg-data"], "img.jpg", { type: "image/jpeg" });
      await service.uploadFiles([file]);

      expect(instance.open).toHaveBeenCalledWith("PUT", "https://r2/img", true);
      expect(instance.setRequestHeader).toHaveBeenCalledWith("Content-Type", "image/jpeg");
    });

    it("reports per-file progress via callback", async () => {
      const session: UploadSession = {
        sessionId: "session-456",
        urls: [
          { name: "b.jpg", key: "uploads/session-456/b.jpg", url: "https://r2/b" },
        ],
        expiresAt: Date.now() + 900_000,
      };
      mockGenerateUploadUrls.mockResolvedValueOnce(session);

      const { instance, fireUploadProgress, fireLoad } = installMockXhr();

      const progressUpdates: FileUploadProgress[][] = [];
      const onProgress = (files: FileUploadProgress[]) => {
        progressUpdates.push(files);
      };

      const file = new File(["test-data"], "b.jpg", { type: "image/jpeg" });

      instance.send.mockImplementation(() => {
        fireUploadProgress(50, 100);
        instance.status = 200;
        fireLoad();
      });

      await service.uploadFiles([file], onProgress);

      // Should have progress updates including completed
      expect(progressUpdates.length).toBeGreaterThanOrEqual(3);

      const lastUpdate = progressUpdates[progressUpdates.length - 1];
      expect(lastUpdate[0].status).toBe("completed");
    });

    it("marks file as failed on XHR error", async () => {
      const session: UploadSession = {
        sessionId: "session-err",
        urls: [
          { name: "c.csv", key: "uploads/session-err/c.csv", url: "https://r2/c" },
        ],
        expiresAt: Date.now() + 900_000,
      };
      mockGenerateUploadUrls.mockResolvedValueOnce(session);

      const { instance, fireError } = installMockXhr();

      const progressUpdates: FileUploadProgress[][] = [];
      const onProgress = (files: FileUploadProgress[]) => {
        progressUpdates.push(files);
      };

      const file = new File(["data"], "c.csv", { type: "text/csv" });

      instance.send.mockImplementation(() => fireError());

      await expect(service.uploadFiles([file], onProgress)).rejects.toThrow(
        "Upload failed",
      );

      const lastUpdate = progressUpdates[progressUpdates.length - 1];
      expect(lastUpdate[0].status).toBe("failed");
      expect(lastUpdate[0].error).toBe("Upload failed");
    });

    it("uses application/octet-stream for files with no type", async () => {
      const session: UploadSession = {
        sessionId: "session-notype",
        urls: [
          { name: "data", key: "uploads/session-notype/data", url: "https://r2/d" },
        ],
        expiresAt: Date.now() + 900_000,
      };
      mockGenerateUploadUrls.mockResolvedValueOnce(session);

      const { instance, fireLoad } = installMockXhr();
      instance.send.mockImplementation(() => {
        instance.status = 200;
        fireLoad();
      });

      const file = new File(["data"], "data", { type: "" });

      await service.uploadFiles([file]);

      expect(mockGenerateUploadUrls).toHaveBeenCalledWith([
        expect.objectContaining({ contentType: "application/octet-stream" }),
      ]);
    });

    it("rejects when HTTP status is not 2xx", async () => {
      const session: UploadSession = {
        sessionId: "session-403",
        urls: [
          { name: "e.png", key: "uploads/session-403/e.png", url: "https://r2/e" },
        ],
        expiresAt: Date.now() + 900_000,
      };
      mockGenerateUploadUrls.mockResolvedValueOnce(session);

      const { instance, fireLoad } = installMockXhr();
      instance.send.mockImplementation(() => {
        instance.status = 403;
        fireLoad();
      });

      const file = new File(["data"], "e.png", { type: "image/png" });

      await expect(service.uploadFiles([file])).rejects.toThrow(
        "Upload failed with status 403",
      );
    });
  });
});
