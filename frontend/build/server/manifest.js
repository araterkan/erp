const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set([]),
	mimeTypes: {},
	_: {
		client: {start:"_app/immutable/entry/start.DAAVhEUS.js",app:"_app/immutable/entry/app.CxgZfNPP.js",imports:["_app/immutable/entry/start.DAAVhEUS.js","_app/immutable/chunks/rSwdUCEb.js","_app/immutable/chunks/C7rgOk53.js","_app/immutable/chunks/CA__Ga0j.js","_app/immutable/chunks/CRPqKb_H.js","_app/immutable/chunks/7HHTjTwu.js","_app/immutable/entry/app.CxgZfNPP.js","_app/immutable/chunks/C7rgOk53.js","_app/immutable/chunks/DE4eATX_.js","_app/immutable/chunks/7HHTjTwu.js","_app/immutable/chunks/jvYQ861k.js","_app/immutable/chunks/BRG1PgnC.js","_app/immutable/chunks/CA__Ga0j.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./chunks/0-DQ1zkPB9.js')),
			__memo(() => import('./chunks/1-CPk4Wbv0.js')),
			__memo(() => import('./chunks/2-CnRdlkTe.js')),
			__memo(() => import('./chunks/3-BXwUIBfF.js')),
			__memo(() => import('./chunks/4-Ci2P_JAi.js')),
			__memo(() => import('./chunks/5-B4GnFqzu.js')),
			__memo(() => import('./chunks/6-DRvaXlZw.js')),
			__memo(() => import('./chunks/7-BNaHN14t.js')),
			__memo(() => import('./chunks/8-B3wPncC8.js')),
			__memo(() => import('./chunks/9-BrwTMMQN.js')),
			__memo(() => import('./chunks/10-BT4z_02S.js')),
			__memo(() => import('./chunks/11-pAKSWT5R.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			},
			{
				id: "/bank",
				pattern: /^\/bank\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			},
			{
				id: "/cash",
				pattern: /^\/cash\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 4 },
				endpoint: null
			},
			{
				id: "/crm",
				pattern: /^\/crm\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 5 },
				endpoint: null
			},
			{
				id: "/finance",
				pattern: /^\/finance\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 6 },
				endpoint: null
			},
			{
				id: "/hr",
				pattern: /^\/hr\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 7 },
				endpoint: null
			},
			{
				id: "/invoice",
				pattern: /^\/invoice\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 8 },
				endpoint: null
			},
			{
				id: "/login",
				pattern: /^\/login\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 9 },
				endpoint: null
			},
			{
				id: "/report",
				pattern: /^\/report\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 10 },
				endpoint: null
			},
			{
				id: "/stock",
				pattern: /^\/stock\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 11 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();

const prerendered = new Set([]);

const base = "";

export { base, manifest, prerendered };
//# sourceMappingURL=manifest.js.map
