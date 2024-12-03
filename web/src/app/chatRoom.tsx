"use client";

import React, { useState, useEffect, useRef } from "react";

export const ChatRoom = () => {
    const [messages, setMessages] = useState([]); // チャットメッセージ
    const [page, setPage] = useState(1);         // 現在のページ
    const [hasNext, setHasNext] = useState(true); // 次のページがあるかどうか
    const loaderRef = useRef<HTMLDivElement | null>(null); // 無限スクロールのトリガー

    useEffect(() => {
        const fetchMessages = async () => {
            if (!hasNext) return; // 次のページがない場合はリクエストしない

            // サーバーからデータを取得
            const response = await fetch(`/api/getMessages?page=${page}&pageSize=20`);
            const data = await response.json();

            setMessages((prev) => [...data.messages, ...prev]); // 古いメッセージの上に追加
            setHasNext(data.hasNext);
        };

        fetchMessages();
    }, [page]);

    // スクロール監視
    useEffect(() => {
        const observer = new IntersectionObserver(
            (entries) => {
                if (entries[0].isIntersecting && hasNext) {
                    setPage((prev) => prev + 1); // 次のページを取得
                }
            },
            { threshold: 1.0 }
        );

        if (loaderRef.current) {
            observer.observe(loaderRef.current);
        }

        return () => {
            if (loaderRef.current) {
                observer.unobserve(loaderRef.current);
            }
        };
    }, [loaderRef.current, hasNext]);

    return (
        <div style={{ height: "400px", overflowY: "scroll" }}>
            <ul>
                {messages.map((message, index) => (
                    <li key={index}>
                        <strong>{message.user}:</strong> {message.text}
                    </li>
                ))}
            </ul>
            {hasNext && (
                <div ref={loaderRef} style={{ textAlign: "center", padding: "10px" }}>
                    Loading more messages...
                </div>
            )}
        </div>
    );
}