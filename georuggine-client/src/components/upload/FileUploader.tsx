import React, { useState, useRef } from "react";
import { Upload, FileText, CheckCircle2, AlertCircle, X, Loader2 } from "lucide-react";

interface FileUploaderProps {
    onFileUploaded?: (content: string) => Promise<void> | void;
}

export function FileUploader({ onFileUploaded }: FileUploaderProps) {
    const [selectedFile, setSelectedFile] = useState<File | null>(null);
    const [isUploading, setIsUploading] = useState(false);
    const [statusMessage, setStatusMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);
    const fileInputRef = useRef<HTMLInputElement>(null);

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setStatusMessage(null);
        const file = event.target.files?.[0];
        if (file) {
            if (!file.name.endsWith(".json")) {
                setStatusMessage({ type: "error", text: "Solo file .json ammessi" });
                return;
            }
            setSelectedFile(file);
        }
    };

    const handleClear = () => {
        setSelectedFile(null);
        setStatusMessage(null);
        if (fileInputRef.current) {
            fileInputRef.current.value = "";
        }
    };

    const handleUpload = async () => {
        if (!selectedFile) return;

        try {
            setIsUploading(true);
            setStatusMessage(null);

            // Lettura contenuto file
            const content = await selectedFile.text();

            // Callback al genitore o logica di trasmissione
            if (onFileUploaded) {
                await onFileUploaded(content);
            }

            setStatusMessage({ type: "success", text: "File caricato con successo!" });
        } catch (err: unknown) {
            const msg = err instanceof Error ? err.message : "Errore durante il caricamento";
            setStatusMessage({ type: "error", text: msg });
        } finally {
            setIsUploading(false);
        }
    };

    return (
        <div className="p-2 rounded-2 bg-dark border border-secondary font-monospace sidebar-file-uploader">
            <div className="d-flex align-items-center justify-content-between mb-2">
                <span className="small text-secondary fw-semibold">File Coordinate</span>
                <Upload size={14} className="text-info opacity-75" />
            </div>

            <input
                ref={fileInputRef}
                type="file"
                accept=".json"
                className="d-none"
                onChange={handleFileChange}
            />

            {!selectedFile ? (
                <button
                    type="button"
                    className="btn btn-outline-secondary btn-sm w-100 font-monospace d-flex align-items-center justify-content-center gap-2 py-1"
                    style={{ fontSize: "12px" }}
                    onClick={() => fileInputRef.current?.click()}
                >
                    <FileText size={14} />
                    <span>Scegli .json</span>
                </button>
            ) : (
                <div className="d-flex flex-column gap-1">
                    <div className="d-flex align-items-center justify-content-between bg-black bg-opacity-50 p-1 px-2 rounded border border-secondary border-opacity-50">
                        <span
                            className="text-truncate text-light small"
                            style={{ maxWidth: "140px", fontSize: "11px" }}
                            title={selectedFile.name}
                        >
                            {selectedFile.name}
                        </span>
                        <button
                            type="button"
                            className="btn btn-link text-secondary p-0 border-0"
                            onClick={handleClear}
                            title="Rimuovi file"
                        >
                            <X size={14} />
                        </button>
                    </div>

                    <div className="d-flex justify-content-between text-secondary" style={{ fontSize: "10px" }}>
                        <span>{(selectedFile.size / 1024).toFixed(1)} KB</span>
                    </div>

                    <button
                        type="button"
                        className="btn btn-primary btn-sm w-100 font-monospace py-1 mt-1 d-flex align-items-center justify-content-center gap-1"
                        style={{ fontSize: "11px" }}
                        onClick={handleUpload}
                        disabled={isUploading}
                    >
                        {isUploading ? (
                            <>
                                <Loader2 size={12} className="spinner-border spinner-border-sm border-0" />
                                <span>Invio...</span>
                            </>
                        ) : (
                            <span>Carica coordinate</span>
                        )}
                    </button>
                </div>
            )}

            {statusMessage && (
                <div
                    className={`d-flex align-items-center gap-1 mt-2 small ${statusMessage.type === "success" ? "text-success" : "text-danger"
                        }`}
                    style={{ fontSize: "11px" }}
                >
                    {statusMessage.type === "success" ? <CheckCircle2 size={12} /> : <AlertCircle size={12} />}
                    <span className="text-truncate">{statusMessage.text}</span>
                </div>
            )}
        </div>
    );
}
