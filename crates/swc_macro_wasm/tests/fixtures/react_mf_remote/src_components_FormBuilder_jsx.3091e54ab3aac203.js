"use strict";
(self["webpackChunk_mf_react_remote"] = self["webpackChunk_mf_react_remote"] || []).push([["src_components_FormBuilder_jsx"], {
"./src/components/FormBuilder.jsx": 
/*!****************************************!*\
  !*** ./src/components/FormBuilder.jsx ***!
  \****************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_3__);
function _array_like_to_array(arr, len) {
    if (len == null || len > arr.length) len = arr.length;
    for(var i = 0, arr2 = new Array(len); i < len; i++)arr2[i] = arr[i];
    return arr2;
}
function _array_with_holes(arr) {
    if (Array.isArray(arr)) return arr;
}
function _array_without_holes(arr) {
    if (Array.isArray(arr)) return _array_like_to_array(arr);
}
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
function _iterable_to_array(iter) {
    if (typeof Symbol !== "undefined" && iter[Symbol.iterator] != null || iter["@@iterator"] != null) return Array.from(iter);
}
function _iterable_to_array_limit(arr, i) {
    var _i = arr == null ? null : typeof Symbol !== "undefined" && arr[Symbol.iterator] || arr["@@iterator"];
    if (_i == null) return;
    var _arr = [];
    var _n = true;
    var _d = false;
    var _s, _e;
    try {
        for(_i = _i.call(arr); !(_n = (_s = _i.next()).done); _n = true){
            _arr.push(_s.value);
            if (i && _arr.length === i) break;
        }
    } catch (err) {
        _d = true;
        _e = err;
    } finally{
        try {
            if (!_n && _i["return"] != null) _i["return"]();
        } finally{
            if (_d) throw _e;
        }
    }
    return _arr;
}
function _non_iterable_rest() {
    throw new TypeError("Invalid attempt to destructure non-iterable instance.\\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.");
}
function _non_iterable_spread() {
    throw new TypeError("Invalid attempt to spread non-iterable instance.\\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.");
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
function _object_without_properties(source, excluded) {
    if (source == null) return {};
    var target = _object_without_properties_loose(source, excluded);
    var key, i;
    if (Object.getOwnPropertySymbols) {
        var sourceSymbolKeys = Object.getOwnPropertySymbols(source);
        for(i = 0; i < sourceSymbolKeys.length; i++){
            key = sourceSymbolKeys[i];
            if (excluded.indexOf(key) >= 0) continue;
            if (!Object.prototype.propertyIsEnumerable.call(source, key)) continue;
            target[key] = source[key];
        }
    }
    return target;
}
function _object_without_properties_loose(source, excluded) {
    if (source == null) return {};
    var target = {};
    var sourceKeys = Object.keys(source);
    var key, i;
    for(i = 0; i < sourceKeys.length; i++){
        key = sourceKeys[i];
        if (excluded.indexOf(key) >= 0) continue;
        target[key] = source[key];
    }
    return target;
}
function _sliced_to_array(arr, i) {
    return _array_with_holes(arr) || _iterable_to_array_limit(arr, i) || _unsupported_iterable_to_array(arr, i) || _non_iterable_rest();
}
function _to_consumable_array(arr) {
    return _array_without_holes(arr) || _iterable_to_array(arr) || _unsupported_iterable_to_array(arr) || _non_iterable_spread();
}
function _unsupported_iterable_to_array(o, minLen) {
    if (!o) return;
    if (typeof o === "string") return _array_like_to_array(o, minLen);
    var n = Object.prototype.toString.call(o).slice(8, -1);
    if (n === "Object" && o.constructor) n = o.constructor.name;
    if (n === "Map" || n === "Set") return Array.from(n);
    if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)) return _array_like_to_array(o, minLen);
}




var FormBuilder = function(param) {
    var _param_fields = param.fields, fields = _param_fields === void 0 ? [] : _param_fields, onSubmit = param.onSubmit, _param_initialValues = param.initialValues, initialValues = _param_initialValues === void 0 ? {} : _param_initialValues, _param_layout = param.layout, layout = _param_layout === void 0 ? 'vertical' : _param_layout;
    var _Form_useForm = _sliced_to_array(antd__WEBPACK_IMPORTED_MODULE_1__.Form.useForm(), 1), form = _Form_useForm[0];
    var handleSubmit = function(values) {
        if (onSubmit) {
            onSubmit(values);
        }
    };
    var handleReset = function() {
        form.resetFields();
    };
    var renderField = function(field) {
        var type = field.type, name = field.name, label = field.label, placeholder = field.placeholder, _field_required = field.required, required = _field_required === void 0 ? false : _field_required, _field_rules = field.rules, rules = _field_rules === void 0 ? [] : _field_rules, _field_options = field.options, options = _field_options === void 0 ? [] : _field_options, restProps = _object_without_properties(field, [
            "type",
            "name",
            "label",
            "placeholder",
            "required",
            "rules",
            "options"
        ]);
        var baseRules = _to_consumable_array(required ? [
            {
                required: true,
                message: "Please provide ".concat(label)
            }
        ] : []).concat(_to_consumable_array(rules));
        switch(type){
            case 'text':
            case 'email':
            case 'url':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
                    key: name,
                    name: name,
                    label: label,
                    rules: _to_consumable_array(baseRules).concat(_to_consumable_array(type === 'email' ? [
                        {
                            type: 'email',
                            message: 'Please enter a valid email'
                        }
                    ] : []), _to_consumable_array(type === 'url' ? [
                        {
                            type: 'url',
                            message: 'Please enter a valid URL'
                        }
                    ] : []))
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Input, _object_spread({
                    type: type,
                    placeholder: placeholder || "Enter ".concat(label)
                }, restProps)));
            case 'password':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
                    key: name,
                    name: name,
                    label: label,
                    rules: baseRules
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Input.Password, _object_spread({
                    placeholder: placeholder || "Enter ".concat(label)
                }, restProps)));
            case 'textarea':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
                    key: name,
                    name: name,
                    label: label,
                    rules: baseRules
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Input.TextArea, _object_spread({
                    rows: 4,
                    placeholder: placeholder || "Enter ".concat(label)
                }, restProps)));
            case 'select':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
                    key: name,
                    name: name,
                    label: label,
                    rules: baseRules
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select, _object_spread({
                    placeholder: placeholder || "Select ".concat(label)
                }, restProps), options.map(function(option) {
                    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
                        key: typeof option === 'string' ? option : option.value,
                        value: typeof option === 'string' ? option : option.value
                    }, typeof option === 'string' ? (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.capitalize)(option) : option.label);
                })));
            case 'date':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
                    key: name,
                    name: name,
                    label: label,
                    rules: baseRules
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.DatePicker, _object_spread({
                    style: {
                        width: '100%'
                    },
                    placeholder: placeholder || "Select ".concat(label)
                }, restProps)));
            case 'checkbox':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
                    key: name,
                    name: name,
                    valuePropName: "checked"
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Checkbox, restProps, label));
            case 'radio':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
                    key: name,
                    name: name,
                    label: label,
                    rules: baseRules
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Radio.Group, restProps, options.map(function(option) {
                    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Radio, {
                        key: typeof option === 'string' ? option : option.value,
                        value: typeof option === 'string' ? option : option.value
                    }, typeof option === 'string' ? (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.capitalize)(option) : option.label);
                })));
            default:
                return null;
        }
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form, {
        form: form,
        layout: layout,
        onFinish: handleSubmit,
        initialValues: initialValues,
        autoComplete: "off"
    }, fields.map(renderField), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
        type: "primary",
        htmlType: "submit",
        icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.SaveOutlined, null)
    }, "Submit"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
        htmlType: "button",
        onClick: handleReset,
        icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.ReloadOutlined, null)
    }, "Reset"))));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (FormBuilder);


}),

}]);