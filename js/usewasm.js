import { useCallback, useState } from "react";
import { simd } from 'wasm-feature-detect';

function useWasm() {
	const [error, setError] = useState(undefined);
	const [wasmInstance, setWasmInstance] = useState(undefined);
	const [loading, setLoading] = useState(true);
	const initialize = useCallback(async () => {
		try {
			setLoading(true);
			setError(undefined);
			const wasm = await import("../pkg/index.js");
			if (await simd()){
				console.log("simd here")
			}
			setWasmInstance(wasm);
		} catch (error) {
			setError(error);
		} finally {
			setLoading(false);
		}
	}, []);

	return {
		error,
		loading,
		initialize,
		wasmInstance,
	};
}

export default useWasm;