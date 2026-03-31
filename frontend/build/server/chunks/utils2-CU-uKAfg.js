function formatCurrency(amount, currency = "USD") {
  return new Intl.NumberFormat("en-US", { style: "currency", currency }).format(amount || 0);
}

export { formatCurrency as f };
//# sourceMappingURL=utils2-CU-uKAfg.js.map
