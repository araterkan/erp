# ERP Sistemi

Rust (Axum) backend ve SvelteKit frontend kullanan kapsamlı bir Kurumsal Kaynak Planlama (ERP) sistemi.

## Modüller

| Modül | Açıklama |
|-------|----------|
| **Finans** | Hesap planı, yevmiye defteri, döviz yönetimi |
| **Stok** | Ürünler, depolar, stok hareketleri, seri/lot takibi |
| **İnsan Kaynakları** | Personel, bordro, izin, puantaj |
| **CRM** | Cari hesaplar, kişiler, görevler |
| **Faturalama** | Satış/alış faturaları, sevkiyat, ödeme planları |
| **Banka & Kasa** | Banka hesapları, kasa işlemleri |
| **Çek/Senet** | Alınan/verilen çek takibi |

## Mimari

```
┌─────────────────┐     HTTP/JSON      ┌──────────────────┐
│  SvelteKit      │ ◄────────────────► │  Rust Axum API   │
│  Frontend :3000 │                    │  Backend  :8080  │
└─────────────────┘                    └────────┬─────────┘
                                                │
                                       ┌────────▼─────────┐
                                       │  PostgreSQL 16   │
                                       │  Database  :5432 │
                                       └──────────────────┘
```

## Hızlı Başlangıç (Docker)

```bash
# Repoyu klonla
git clone <repo-url>
cd erp

# Ortam değişkenlerini ayarla
cp .env.example .env
# .env dosyasını düzenle (özellikle JWT_SECRET)

# Docker ile başlat
docker-compose up -d

# Logları takip et
docker-compose logs -f
```

Servisler:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
- Varsayılan kullanıcı: `admin` / `admin`

## Geliştirme Ortamı

### Gereksinimler
- Rust 1.80+
- Node.js 20+
- PostgreSQL 16+

### Backend

```bash
cd backend
cp ../.env.example .env
# DATABASE_URL ayarla

cargo run
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

### Veritabanı Migration

```bash
# sqlx-cli ile
cargo install sqlx-cli
sqlx migrate run --database-url postgres://erp_user:erp_password@localhost:5432/erp

# Ya da doğrudan psql ile
psql -U erp_user -d erp -f migrations/001_initial.sql
```

## Ortam Değişkenleri

| Değişken | Açıklama | Varsayılan |
|----------|----------|-----------|
| `DATABASE_URL` | PostgreSQL bağlantı URL'i | - |
| `JWT_SECRET` | JWT imzalama anahtarı | - |
| `PORT` | Backend port | `8080` |
| `RUST_LOG` | Log seviyesi | `info` |
| `PUBLIC_API_URL` | Frontend'in API URL'i | `http://localhost:8080` |
| `ORIGIN` | SvelteKit origin | `http://localhost:3000` |

## API Genel Bakış

Tüm endpoint'ler `/api/v1` prefix'i ile başlar.

| Endpoint | Açıklama |
|----------|----------|
| `POST /api/v1/auth/login` | Kullanıcı girişi, JWT döner |
| `POST /api/v1/auth/logout` | Oturum kapatma |
| `GET  /api/v1/users` | Kullanıcı listesi (admin) |
| `GET  /api/v1/products` | Ürün listesi |
| `GET  /api/v1/invoices` | Fatura listesi |
| `GET  /api/v1/employees` | Personel listesi |
| `GET  /api/v1/accounts` | Cari hesap listesi |
| `GET  /api/v1/finance/journal` | Yevmiye kayıtları |

## Lisans

MIT
