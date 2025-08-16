"use strict";
(self["webpackChunk_mf_react_remote"] = self["webpackChunk_mf_react_remote"] || []).push([["src_store_slices_js"], {
"./src/store/slices.js": 
/*!*****************************!*\
  !*** ./src/store/slices.js ***!
  \*****************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  addNotification: () => (addNotification),
  clearNotifications: () => (clearNotifications),
  "default": () => (__WEBPACK_DEFAULT_EXPORT__),
  notificationSlice: () => (notificationSlice),
  removeNotification: () => (removeNotification),
  setPrimaryColor: () => (setPrimaryColor),
  themeSlice: () => (themeSlice),
  toggleTheme: () => (toggleTheme)
});
/* ESM import */var _reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! @reduxjs/toolkit */ "webpack/sharing/consume/default/@reduxjs/toolkit/@reduxjs/toolkit");
function _define_property(obj, key, value) {
    if (key in obj) {
        Object.defineProperty(obj, key, {
            value: value,
            enumerable: true,
            configurable: true,
            writable: true
        });
    } else {
        obj[key] = value;
    }
    return obj;
}
function _object_spread(target) {
    for(var i = 1; i < arguments.length; i++){
        var source = arguments[i] != null ? arguments[i] : {};
        var ownKeys = Object.keys(source);
        if (typeof Object.getOwnPropertySymbols === "function") {
            ownKeys = ownKeys.concat(Object.getOwnPropertySymbols(source).filter(function(sym) {
                return Object.getOwnPropertyDescriptor(source, sym).enumerable;
            }));
        }
        ownKeys.forEach(function(key) {
            _define_property(target, key, source[key]);
        });
    }
    return target;
}
function ownKeys(object, enumerableOnly) {
    var keys = Object.keys(object);
    if (Object.getOwnPropertySymbols) {
        var symbols = Object.getOwnPropertySymbols(object);
        if (enumerableOnly) {
            symbols = symbols.filter(function(sym) {
                return Object.getOwnPropertyDescriptor(object, sym).enumerable;
            });
        }
        keys.push.apply(keys, symbols);
    }
    return keys;
}
function _object_spread_props(target, source) {
    source = source != null ? source : {};
    if (Object.getOwnPropertyDescriptors) {
        Object.defineProperties(target, Object.getOwnPropertyDescriptors(source));
    } else {
        ownKeys(Object(source)).forEach(function(key) {
            Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key));
        });
    }
    return target;
}

// Shared notification slice
var notificationSlice = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createSlice)({
    name: 'notifications',
    initialState: {
        messages: []
    },
    reducers: {
        addNotification: function(state, action) {
            state.messages.push(_object_spread_props(_object_spread({
                id: Date.now()
            }, action.payload), {
                timestamp: new Date().toISOString()
            }));
        },
        removeNotification: function(state, action) {
            state.messages = state.messages.filter(function(msg) {
                return msg.id !== action.payload;
            });
        },
        clearNotifications: function(state) {
            state.messages = [];
        }
    }
});
// Shared theme slice
var themeSlice = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createSlice)({
    name: 'theme',
    initialState: {
        mode: 'light',
        primaryColor: '#1890ff'
    },
    reducers: {
        toggleTheme: function(state) {
            state.mode = state.mode === 'light' ? 'dark' : 'light';
        },
        setPrimaryColor: function(state, action) {
            state.primaryColor = action.payload;
        }
    }
});
var _notificationSlice_actions = notificationSlice.actions;
var addNotification = _notificationSlice_actions.addNotification, removeNotification = _notificationSlice_actions.removeNotification, clearNotifications = _notificationSlice_actions.clearNotifications;
var _themeSlice_actions = themeSlice.actions;
var toggleTheme = _themeSlice_actions.toggleTheme, setPrimaryColor = _themeSlice_actions.setPrimaryColor;
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = ({
    notifications: notificationSlice.reducer,
    theme: themeSlice.reducer
});


}),

}]);