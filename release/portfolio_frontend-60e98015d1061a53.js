let X=0,S=null,U=`undefined`,$=`boolean`,Q=128,a0=`string`,T=1,a1=`Object`,V=`utf-8`,_=`number`,a3=4,Z=`function`,P=Array,W=Error,a2=FinalizationRegistry,a4=Object,Y=Uint8Array,R=undefined;var J=(async(a,b)=>{if(typeof Response===Z&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===Z){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var v=(a=>{const b=typeof a;if(b==_||b==$||a==S){return `${a}`};if(b==a0){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==S){return `Symbol`}else{return `Symbol(${b})`}};if(b==Z){const b=a.name;if(typeof b==a0&&b.length>X){return `Function(${b})`}else{return `Function`}};if(P.isArray(a)){const b=a.length;let c=`[`;if(b>X){c+=v(a[X])};for(let d=T;d<b;d++){c+=`, `+ v(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>T){d=c[T]}else{return toString.call(a)};if(d==a1){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return a1}};if(a instanceof W){return `${a.name}: ${a.message}\n${a.stack}`};return d});var L=((a,b)=>{});var H=((a,b)=>{a=a>>>X;const c=G();const d=c.subarray(a/a3,a/a3+ b);const e=[];for(let a=X;a<d.length;a++){e.push(g(d[a]))};return e});var h=(a=>{if(e===c.length)c.push(c.length+ T);const b=e;e=c[b];c[b]=a;return b});var g=(a=>{const b=d(a);f(a);return b});var O=(async(a)=>{if(b!==R)return b;if(typeof a===U){a=new URL(`portfolio_frontend-60e98015d1061a53_bg.wasm`,import.meta.url)};const c=K();if(typeof a===a0||typeof Request===Z&&a instanceof Request||typeof URL===Z&&a instanceof URL){a=fetch(a)};L(c);const {instance:d,module:e}=await J(await a,c);return M(d,e)});var N=(a=>{if(b!==R)return b;const c=K();L(c);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const d=new WebAssembly.Instance(a,c);return M(d,a)});var M=((a,c)=>{b=a.exports;O.__wbindgen_wasm_module=c;t=S;r=S;F=S;j=S;b.__wbindgen_start();return b});var A=((a,d,e)=>{try{b.wasm_bindgen__convert__closures__invoke1_ref__hfe4c232425f62f53(a,d,z(e))}finally{c[y++]=R}});function I(a,c){try{return a.apply(this,c)}catch(a){b.__wbindgen_exn_store(h(a))}}var K=(()=>{const c={};c.wbg={};c.wbg.__wbindgen_object_drop_ref=(a=>{g(a)});c.wbg.__wbindgen_object_clone_ref=(a=>{const b=d(a);return h(b)});c.wbg.__wbg_initSyncGame_16a9f56eb5089614=((b,c)=>{a(E(b,c))});c.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{d(a).__yew_listener_id=b>>>X});c.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const c=d(b).__yew_listener_id;s()[a/a3+ T]=q(c)?X:c;s()[a/a3+ X]=!q(c)});c.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const c=d(b).__yew_subtree_id;s()[a/a3+ T]=q(c)?X:c;s()[a/a3+ X]=!q(c)});c.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{d(a).__yew_subtree_id=b>>>X});c.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const c=d(b).__yew_subtree_cache_key;s()[a/a3+ T]=q(c)?X:c;s()[a/a3+ X]=!q(c)});c.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{d(a).__yew_subtree_cache_key=b>>>X});c.wbg.__wbindgen_string_new=((a,b)=>{const c=l(a,b);return h(c)});c.wbg.__wbindgen_cb_drop=(a=>{const b=g(a).original;if(b.cnt--==T){b.a=X;return !0};const c=!1;return c});c.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new W();return h(a)});c.wbg.__wbg_stack_658279fe44541cf6=((a,c)=>{const e=d(c).stack;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_error_f851667af71bcfc6=((a,c)=>{let d;let e;try{d=a;e=c;console.error(l(a,c))}finally{b.__wbindgen_free(d,e,T)}});c.wbg.__wbg_queueMicrotask_f61ee94ee663068b=(a=>{queueMicrotask(d(a))});c.wbg.__wbg_queueMicrotask_f82fc5d1e8f816ae=(a=>{const b=d(a).queueMicrotask;return h(b)});c.wbg.__wbindgen_is_function=(a=>{const b=typeof d(a)===Z;return b});c.wbg.__wbindgen_string_get=((a,c)=>{const e=d(c);const f=typeof e===a0?e:R;var g=q(f)?X:p(f,b.__wbindgen_malloc,b.__wbindgen_realloc);var h=m;s()[a/a3+ T]=h;s()[a/a3+ X]=g});c.wbg.__wbindgen_is_string=(a=>{const b=typeof d(a)===a0;return b});c.wbg.__wbindgen_is_object=(a=>{const b=d(a);const c=typeof b===`object`&&b!==S;return c});c.wbg.__wbindgen_is_undefined=(a=>{const b=d(a)===R;return b});c.wbg.__wbindgen_in=((a,b)=>{const c=d(a) in d(b);return c});c.wbg.__wbindgen_error_new=((a,b)=>{const c=new W(l(a,b));return h(c)});c.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const c=d(a)==d(b);return c});c.wbg.__wbindgen_boolean_get=(a=>{const b=d(a);const c=typeof b===$?(b?T:X):2;return c});c.wbg.__wbindgen_number_get=((a,b)=>{const c=d(b);const e=typeof c===_?c:R;u()[a/8+ T]=q(e)?X:e;s()[a/a3+ X]=!q(e)});c.wbg.__wbindgen_as_number=(a=>{const b=+d(a);return b});c.wbg.__wbg_getwithrefkey_4a92a5eca60879b9=((a,b)=>{const c=d(a)[d(b)];return h(c)});c.wbg.__wbg_error_a526fb08a0205972=((a,c)=>{var d=H(a,c).slice();b.__wbindgen_free(a,c*a3,a3);console.error(...d)});c.wbg.__wbg_body_874ccb42daaab363=(a=>{const b=d(a).body;return q(b)?X:h(b)});c.wbg.__wbg_createElement_03cf347ddad1c8c0=function(){return I(((a,b,c)=>{const e=d(a).createElement(l(b,c));return h(e)}),arguments)};c.wbg.__wbg_createElementNS_93f8de4acdef6da8=function(){return I(((a,b,c,e,f)=>{const g=d(a).createElementNS(b===X?R:l(b,c),l(e,f));return h(g)}),arguments)};c.wbg.__wbg_createTextNode_ea32ad2506f7ae78=((a,b,c)=>{const e=d(a).createTextNode(l(b,c));return h(e)});c.wbg.__wbg_querySelector_118a0639aa1f51cd=function(){return I(((a,b,c)=>{const e=d(a).querySelector(l(b,c));return q(e)?X:h(e)}),arguments)};c.wbg.__wbg_instanceof_Element_813f33306edae612=(a=>{let b;try{b=d(a) instanceof Element}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_namespaceURI_230708ae7f4baac5=((a,c)=>{const e=d(c).namespaceURI;var f=q(e)?X:p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);var g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_setinnerHTML_95222f1a2e797983=((a,b,c)=>{d(a).innerHTML=l(b,c)});c.wbg.__wbg_outerHTML_eb21059e86b1e9f4=((a,c)=>{const e=d(c).outerHTML;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_removeAttribute_0c021c98a4dc7402=function(){return I(((a,b,c)=>{d(a).removeAttribute(l(b,c))}),arguments)};c.wbg.__wbg_setAttribute_f7ffa687ef977957=function(){return I(((a,b,c,e,f)=>{d(a).setAttribute(l(b,c),l(e,f))}),arguments)};c.wbg.__wbg_instanceof_Window_cee7a886d55e7df5=(a=>{let b;try{b=d(a) instanceof Window}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_document_eb7fd66bde3ee213=(a=>{const b=d(a).document;return q(b)?X:h(b)});c.wbg.__wbg_location_b17760ac7977a47a=(a=>{const b=d(a).location;return h(b)});c.wbg.__wbg_history_6882f83324841599=function(){return I((a=>{const b=d(a).history;return h(b)}),arguments)};c.wbg.__wbg_setchecked_50e21357d62a8ccd=((a,b)=>{d(a).checked=b!==X});c.wbg.__wbg_value_99f5294791d62576=((a,c)=>{const e=d(c).value;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_setvalue_bba31de32cdbb32c=((a,b,c)=>{d(a).value=l(b,c)});c.wbg.__wbg_value_ffef403d62e3df58=((a,c)=>{const e=d(c).value;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_setvalue_cbab536654d8dd52=((a,b,c)=>{d(a).value=l(b,c)});c.wbg.__wbg_state_dce1712758f75ed1=function(){return I((a=>{const b=d(a).state;return h(b)}),arguments)};c.wbg.__wbg_instanceof_ShadowRoot_ef56f954a86c7472=(a=>{let b;try{b=d(a) instanceof ShadowRoot}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_host_dfffc3b2ba786fb8=(a=>{const b=d(a).host;return h(b)});c.wbg.__wbg_parentNode_e3a5ee563364a613=(a=>{const b=d(a).parentNode;return q(b)?X:h(b)});c.wbg.__wbg_parentElement_45a9756dc74ff48b=(a=>{const b=d(a).parentElement;return q(b)?X:h(b)});c.wbg.__wbg_childNodes_535387effca84f4e=(a=>{const b=d(a).childNodes;return h(b)});c.wbg.__wbg_lastChild_d22dbf81f92f163b=(a=>{const b=d(a).lastChild;return q(b)?X:h(b)});c.wbg.__wbg_nextSibling_87d2b32dfbf09fe3=(a=>{const b=d(a).nextSibling;return q(b)?X:h(b)});c.wbg.__wbg_setnodeValue_d1cec51282858afe=((a,b,c)=>{d(a).nodeValue=b===X?R:l(b,c)});c.wbg.__wbg_textContent_528ff517a0418a3e=((a,c)=>{const e=d(c).textContent;var f=q(e)?X:p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);var g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_cloneNode_ea49a704f0699b2e=function(){return I((a=>{const b=d(a).cloneNode();return h(b)}),arguments)};c.wbg.__wbg_insertBefore_2be91083083caa9e=function(){return I(((a,b,c)=>{const e=d(a).insertBefore(d(b),d(c));return h(e)}),arguments)};c.wbg.__wbg_removeChild_660924798c7e128c=function(){return I(((a,b)=>{const c=d(a).removeChild(d(b));return h(c)}),arguments)};c.wbg.__wbg_bubbles_31126fc08276cf99=(a=>{const b=d(a).bubbles;return b});c.wbg.__wbg_cancelBubble_ae95595adf5ae83d=(a=>{const b=d(a).cancelBubble;return b});c.wbg.__wbg_composedPath_bd8a0336a042e053=(a=>{const b=d(a).composedPath();return h(b)});c.wbg.__wbg_href_6918c551c13f118b=((a,c)=>{const e=d(c).href;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_href_a5b902312c18d121=function(){return I(((a,c)=>{const e=d(c).href;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f}),arguments)};c.wbg.__wbg_pathname_d98d0a003b664ef0=function(){return I(((a,c)=>{const e=d(c).pathname;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f}),arguments)};c.wbg.__wbg_search_40927d5af164fdfe=function(){return I(((a,c)=>{const e=d(c).search;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f}),arguments)};c.wbg.__wbg_hash_163703b5971e593c=function(){return I(((a,c)=>{const e=d(c).hash;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f}),arguments)};c.wbg.__wbg_pathname_3bec400c9c042d62=((a,c)=>{const e=d(c).pathname;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_search_6b70a3bf2ceb3f63=((a,c)=>{const e=d(c).search;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_hash_6169ffe1f1446fd4=((a,c)=>{const e=d(c).hash;const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbg_new_79acf9a4da56c772=function(){return I(((a,b)=>{const c=new URL(l(a,b));return h(c)}),arguments)};c.wbg.__wbg_newwithbase_98813076a95cdc23=function(){return I(((a,b,c,d)=>{const e=new URL(l(a,b),l(c,d));return h(e)}),arguments)};c.wbg.__wbg_addEventListener_bc4a7ad4cc72c6bf=function(){return I(((a,b,c,e,f)=>{d(a).addEventListener(l(b,c),d(e),d(f))}),arguments)};c.wbg.__wbg_removeEventListener_deae10c75ef836f8=function(){return I(((a,b,c,e,f)=>{d(a).removeEventListener(l(b,c),d(e),f!==X)}),arguments)};c.wbg.__wbg_get_0ee8ea3c7c984c45=((a,b)=>{const c=d(a)[b>>>X];return h(c)});c.wbg.__wbg_length_161c0d89c6535c1d=(a=>{const b=d(a).length;return b});c.wbg.__wbg_newnoargs_cfecb3965268594c=((a,b)=>{const c=new Function(l(a,b));return h(c)});c.wbg.__wbg_call_3f093dd26d5569f8=function(){return I(((a,b)=>{const c=d(a).call(d(b));return h(c)}),arguments)};c.wbg.__wbg_new_632630b5cec17f21=(()=>{const a=new a4();return h(a)});c.wbg.__wbg_self_05040bd9523805b9=function(){return I((()=>{const a=self.self;return h(a)}),arguments)};c.wbg.__wbg_window_adc720039f2cb14f=function(){return I((()=>{const a=window.window;return h(a)}),arguments)};c.wbg.__wbg_globalThis_622105db80c1457d=function(){return I((()=>{const a=globalThis.globalThis;return h(a)}),arguments)};c.wbg.__wbg_global_f56b013ed9bcf359=function(){return I((()=>{const a=global.global;return h(a)}),arguments)};c.wbg.__wbg_from_58c79ccfb68060f5=(a=>{const b=P.from(d(a));return h(b)});c.wbg.__wbg_instanceof_ArrayBuffer_9221fa854ffb71b5=(a=>{let b;try{b=d(a) instanceof ArrayBuffer}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_isSafeInteger_a23a66ee7c41b273=(a=>{const b=Number.isSafeInteger(d(a));return b});c.wbg.__wbg_entries_488960b196cfb6a5=(a=>{const b=a4.entries(d(a));return h(b)});c.wbg.__wbg_is_bd5dc4ae269cba1c=((a,b)=>{const c=a4.is(d(a),d(b));return c});c.wbg.__wbg_resolve_5da6faf2c96fd1d5=(a=>{const b=Promise.resolve(d(a));return h(b)});c.wbg.__wbg_then_f9e58f5a50f43eae=((a,b)=>{const c=d(a).then(d(b));return h(c)});c.wbg.__wbg_buffer_b914fb8b50ebbc3e=(a=>{const b=d(a).buffer;return h(b)});c.wbg.__wbg_new_b1f2d6842d615181=(a=>{const b=new Y(d(a));return h(b)});c.wbg.__wbg_set_7d988c98e6ced92d=((a,b,c)=>{d(a).set(d(b),c>>>X)});c.wbg.__wbg_length_21c4b0ae73cba59d=(a=>{const b=d(a).length;return b});c.wbg.__wbg_instanceof_Uint8Array_c299a4ee232e76ba=(a=>{let b;try{b=d(a) instanceof Y}catch(a){b=!1}const c=b;return c});c.wbg.__wbg_set_961700853a212a39=function(){return I(((a,b,c)=>{const e=Reflect.set(d(a),d(b),d(c));return e}),arguments)};c.wbg.__wbindgen_debug_string=((a,c)=>{const e=v(d(c));const f=p(e,b.__wbindgen_malloc,b.__wbindgen_realloc);const g=m;s()[a/a3+ T]=g;s()[a/a3+ X]=f});c.wbg.__wbindgen_throw=((a,b)=>{throw new W(l(a,b))});c.wbg.__wbindgen_memory=(()=>{const a=b.memory;return h(a)});c.wbg.__wbindgen_closure_wrapper1151=((a,b,c)=>{const d=x(a,b,573,A);return h(d)});c.wbg.__wbindgen_closure_wrapper1252=((a,b,c)=>{const d=B(a,b,610,C);return h(d)});c.wbg.__wbindgen_closure_wrapper1300=((a,b,c)=>{const d=B(a,b,633,D);return h(d)});return c});var D=((a,d,e)=>{try{b.wasm_bindgen__convert__closures__invoke1_mut_ref__h19d4784ed2393697(a,d,z(e))}finally{c[y++]=R}});var s=(()=>{if(r===S||r.byteLength===X){r=new Int32Array(b.memory.buffer)};return r});var q=(a=>a===R||a===S);var d=(a=>c[a]);var G=(()=>{if(F===S||F.byteLength===X){F=new Uint32Array(b.memory.buffer)};return F});var E=((a,b)=>{a=a>>>X;return k().subarray(a/T,a/T+ b)});var C=((a,c,d)=>{b._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0f8aa5fe587b49c2(a,c,h(d))});var u=(()=>{if(t===S||t.byteLength===X){t=new Float64Array(b.memory.buffer)};return t});var x=((a,c,d,e)=>{const f={a:a,b:c,cnt:T,dtor:d};const g=(...a)=>{f.cnt++;try{return e(f.a,f.b,...a)}finally{if(--f.cnt===X){b.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=X;w.unregister(f)}}};g.original=f;w.register(g,f,f);return g});var f=(a=>{if(a<132)return;c[a]=e;e=a});var B=((a,c,d,e)=>{const f={a:a,b:c,cnt:T,dtor:d};const g=(...a)=>{f.cnt++;const c=f.a;f.a=X;try{return e(c,f.b,...a)}finally{if(--f.cnt===X){b.__wbindgen_export_2.get(f.dtor)(c,f.b);w.unregister(f)}else{f.a=c}}};g.original=f;w.register(g,f,f);return g});var p=((a,b,c)=>{if(c===R){const c=n.encode(a);const d=b(c.length,T)>>>X;k().subarray(d,d+ c.length).set(c);m=c.length;return d};let d=a.length;let e=b(d,T)>>>X;const f=k();let g=X;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==X){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,T)>>>X;const b=k().subarray(e+ g,e+ d);const f=o(a,b);g+=f.written;e=c(e,d,g,T)>>>X};m=g;return e});var k=(()=>{if(j===S||j.byteLength===X){j=new Y(b.memory.buffer)};return j});var l=((a,b)=>{a=a>>>X;return i.decode(k().subarray(a,a+ b))});var z=(a=>{if(y==T)throw new W(`out of js stack`);c[--y]=a;return y});import{initSyncGame as a}from"./snippets/portfolio_frontend-f4568b64a6615282/games/first/mygame.js";let b;const c=new P(Q).fill(R);c.push(R,S,!0,!1);let e=c.length;const i=typeof TextDecoder!==U?new TextDecoder(V,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw W(`TextDecoder not available`)}};if(typeof TextDecoder!==U){i.decode()};let j=S;let m=X;const n=typeof TextEncoder!==U?new TextEncoder(V):{encode:()=>{throw W(`TextEncoder not available`)}};const o=typeof n.encodeInto===Z?((a,b)=>n.encodeInto(a,b)):((a,b)=>{const c=n.encode(a);b.set(c);return {read:a.length,written:c.length}});let r=S;let t=S;const w=typeof a2===U?{register:()=>{},unregister:()=>{}}:new a2(a=>{b.__wbindgen_export_2.get(a.dtor)(a.a,a.b)});let y=Q;let F=S;export default O;export{N as initSync}