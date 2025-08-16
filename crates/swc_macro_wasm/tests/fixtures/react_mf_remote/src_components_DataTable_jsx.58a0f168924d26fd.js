"use strict";
(self["webpackChunk_mf_react_remote"] = self["webpackChunk_mf_react_remote"] || []).push([["src_components_DataTable_jsx"], {
"./src/components/DataTable.jsx": 
/*!**************************************!*\
  !*** ./src/components/DataTable.jsx ***!
  \**************************************/
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
/* ESM import */var dayjs__WEBPACK_IMPORTED_MODULE_4__ = /* #__PURE__ */ __webpack_require__(/*! dayjs */ "webpack/sharing/consume/default/dayjs/dayjs");
/* ESM import */var dayjs__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(dayjs__WEBPACK_IMPORTED_MODULE_4__);
function _array_like_to_array(arr, len) {
    if (len == null || len > arr.length) len = arr.length;
    for(var i = 0, arr2 = new Array(len); i < len; i++)arr2[i] = arr[i];
    return arr2;
}
function _array_with_holes(arr) {
    if (Array.isArray(arr)) return arr;
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
function _sliced_to_array(arr, i) {
    return _array_with_holes(arr) || _iterable_to_array_limit(arr, i) || _unsupported_iterable_to_array(arr, i) || _non_iterable_rest();
}
function _unsupported_iterable_to_array(o, minLen) {
    if (!o) return;
    if (typeof o === "string") return _array_like_to_array(o, minLen);
    var n = Object.prototype.toString.call(o).slice(8, -1);
    if (n === "Object" && o.constructor) n = o.constructor.name;
    if (n === "Map" || n === "Set") return Array.from(n);
    if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)) return _array_like_to_array(o, minLen);
}





// Default sample data if none provided - moved outside component to prevent re-creation
var DEFAULT_DATA = [
    {
        key: '1',
        product: 'iPhone 14 Pro',
        category: 'Electronics',
        price: 999,
        stock: 45,
        status: 'available',
        lastUpdated: '2024-01-15'
    },
    {
        key: '2',
        product: 'MacBook Pro M3',
        category: 'Computers',
        price: 2499,
        stock: 12,
        status: 'low-stock',
        lastUpdated: '2024-01-14'
    },
    {
        key: '3',
        product: 'AirPods Pro',
        category: 'Accessories',
        price: 249,
        stock: 0,
        status: 'out-of-stock',
        lastUpdated: '2024-01-13'
    },
    {
        key: '4',
        product: 'iPad Air',
        category: 'Tablets',
        price: 599,
        stock: 67,
        status: 'available',
        lastUpdated: '2024-01-12'
    }
];
var DataTable = function(param) {
    var propData = param.data, propColumns = param.columns;
    var _useState = _sliced_to_array((0,react__WEBPACK_IMPORTED_MODULE_0__.useState)([]), 2), filteredData = _useState[0], setFilteredData = _useState[1];
    var _useState1 = _sliced_to_array((0,react__WEBPACK_IMPORTED_MODULE_0__.useState)(''), 2), searchText = _useState1[0], setSearchText = _useState1[1];
    var _useState2 = _sliced_to_array((0,react__WEBPACK_IMPORTED_MODULE_0__.useState)(''), 2), searchedColumn = _useState2[0], setSearchedColumn = _useState2[1];
    var data = (0,react__WEBPACK_IMPORTED_MODULE_0__.useMemo)(function() {
        return propData || DEFAULT_DATA;
    }, [
        propData
    ]);
    (0,react__WEBPACK_IMPORTED_MODULE_0__.useEffect)(function() {
        setFilteredData(data);
    }, [
        data
    ]);
    var handleSearch = (0,react__WEBPACK_IMPORTED_MODULE_0__.useCallback)((0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.debounce)(function(selectedKeys, confirm, dataIndex) {
        confirm();
        setSearchText(selectedKeys[0]);
        setSearchedColumn(dataIndex);
    }, 300), []);
    var handleReset = (0,react__WEBPACK_IMPORTED_MODULE_0__.useCallback)(function(clearFilters) {
        clearFilters();
        setSearchText('');
    }, []);
    var getColumnSearchProps = function(dataIndex) {
        return {
            filterDropdown: function(param) {
                var setSelectedKeys = param.setSelectedKeys, selectedKeys = param.selectedKeys, confirm = param.confirm, clearFilters = param.clearFilters;
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
                    style: {
                        padding: 8
                    }
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Input, {
                    placeholder: "Search ".concat(dataIndex),
                    value: selectedKeys[0],
                    onChange: function(e) {
                        return setSelectedKeys(e.target.value ? [
                            e.target.value
                        ] : []);
                    },
                    onPressEnter: function() {
                        return handleSearch(selectedKeys, confirm, dataIndex);
                    },
                    style: {
                        marginBottom: 8,
                        display: 'block'
                    }
                }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, null, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
                    type: "primary",
                    onClick: function() {
                        return handleSearch(selectedKeys, confirm, dataIndex);
                    },
                    icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.SearchOutlined, null),
                    size: "small",
                    style: {
                        width: 90
                    }
                }, "Search"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
                    onClick: function() {
                        return handleReset(clearFilters);
                    },
                    size: "small",
                    style: {
                        width: 90
                    }
                }, "Reset")));
            },
            filterIcon: function(filtered) {
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.SearchOutlined, {
                    style: {
                        color: filtered ? '#1890ff' : undefined
                    }
                });
            },
            onFilter: function(value, record) {
                return record[dataIndex].toString().toLowerCase().includes(value.toLowerCase());
            }
        };
    };
    var defaultColumns = [
        _object_spread_props(_object_spread({
            title: 'Product',
            dataIndex: 'product',
            key: 'product'
        }, getColumnSearchProps('product')), {
            sorter: function(a, b) {
                return a.product.localeCompare(b.product);
            }
        }),
        {
            title: 'Category',
            dataIndex: 'category',
            key: 'category',
            filters: [
                {
                    text: 'Electronics',
                    value: 'Electronics'
                },
                {
                    text: 'Computers',
                    value: 'Computers'
                },
                {
                    text: 'Accessories',
                    value: 'Accessories'
                },
                {
                    text: 'Tablets',
                    value: 'Tablets'
                }
            ],
            onFilter: function(value, record) {
                return record.category === value;
            }
        },
        {
            title: 'Price',
            dataIndex: 'price',
            key: 'price',
            render: function(price) {
                return "$".concat(price.toLocaleString());
            },
            sorter: function(a, b) {
                return a.price - b.price;
            }
        },
        {
            title: 'Stock',
            dataIndex: 'stock',
            key: 'stock',
            sorter: function(a, b) {
                return a.stock - b.stock;
            }
        },
        {
            title: 'Status',
            dataIndex: 'status',
            key: 'status',
            render: function(status) {
                var config = {
                    'available': {
                        color: 'green',
                        text: 'Available'
                    },
                    'low-stock': {
                        color: 'orange',
                        text: 'Low Stock'
                    },
                    'out-of-stock': {
                        color: 'red',
                        text: 'Out of Stock'
                    }
                };
                var _ref = config[status] || {
                    color: 'default',
                    text: status
                }, color = _ref.color, text = _ref.text;
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Tag, {
                    color: color
                }, text);
            },
            filters: [
                {
                    text: 'Available',
                    value: 'available'
                },
                {
                    text: 'Low Stock',
                    value: 'low-stock'
                },
                {
                    text: 'Out of Stock',
                    value: 'out-of-stock'
                }
            ],
            onFilter: function(value, record) {
                return record.status === value;
            }
        },
        {
            title: 'Last Updated',
            dataIndex: 'lastUpdated',
            key: 'lastUpdated',
            render: function(date) {
                return dayjs__WEBPACK_IMPORTED_MODULE_4___default()(date).format('MMM D, YYYY');
            },
            sorter: function(a, b) {
                return dayjs__WEBPACK_IMPORTED_MODULE_4___default()(a.lastUpdated).unix() - dayjs__WEBPACK_IMPORTED_MODULE_4___default()(b.lastUpdated).unix();
            }
        }
    ];
    var columns = propColumns || defaultColumns;
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Table, {
        columns: columns,
        dataSource: filteredData,
        pagination: {
            pageSize: 10,
            showSizeChanger: true,
            showTotal: function(total) {
                return "Total ".concat(total, " items");
            }
        },
        scroll: {
            x: true
        }
    });
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (DataTable);


}),

}]);