# Setup Guide - ML DevOps Platform

## Prerequisites

- **Node.js** 18.x or higher
- **Yarn** 1.22.x or higher
- **PostgreSQL** 14.x or higher (provided via Abacus.AI)
- **Git** for version control

## Environment Setup

### 1. Clone and Navigate

```bash
cd /home/ubuntu/ml_devops_platform/nextjs_space
```

### 2. Install Dependencies

```bash
yarn install
```

This installs all required packages including:
- Next.js 14.x
- React 18.x
- Prisma 6.x
- Tailwind CSS 3.x
- Recharts for visualization
- Framer Motion for animations
- Lucide React for icons

### 3. Environment Variables

The `.env` file is automatically configured with:

```env
DATABASE_URL=postgresql://...
ABACUSAI_API_KEY=...
NEXTAUTH_URL=http://localhost:3000
```

**Note**: API keys are pre-configured. No manual setup needed.

### 4. Database Setup

#### Generate Prisma Client

```bash
yarn prisma generate
```

This generates the type-safe Prisma client from `schema.prisma`.

#### Push Schema to Database

```bash
yarn prisma db push
```

This creates the `event_logs` table in PostgreSQL.

#### Seed Database (Optional)

```bash
yarn prisma db seed
```

This populates the database with sample event streams for demonstration.

### 5. Run Development Server

```bash
yarn dev
```

The application will start on [http://localhost:3000](http://localhost:3000).

## Verification Steps

### 1. Test Event Stream

- Open [http://localhost:3000](http://localhost:3000)
- Click "Simulate Streaming Message" in Event Simulator
- Verify tokens stream in the chat interface

### 2. Test Widget System

- Click "Mount Text", "Mount Code", "Mount Status", "Mount Chart"
- Verify widgets appear below the chat
- Click "Unmount Last Widget" to test removal

### 3. Test Event Persistence

- Interact with the platform (send messages, mount widgets)
- Enter a stream name and click "Save"
- Click "Reset" to clear the UI
- Click "Load" on the saved stream
- Verify the UI restores to the saved state

### 4. Test Event Replay

- After loading a stream, adjust replay speed
- Click "Replay"
- Watch the UI reconstruct event-by-event

### 5. Test Chat Streaming

- Type a message in the chat input
- Press Enter or click Send
- Verify AI response streams token-by-token

## Build for Production

### 1. Build the Application

```bash
yarn build
```

This creates an optimized production build in `.next/`.

### 2. Start Production Server

```bash
yarn start
```

The production server will start on [http://localhost:3000](http://localhost:3000).

## Database Management

### View Database

```bash
yarn prisma studio
```

This opens Prisma Studio at [http://localhost:5555](http://localhost:5555) for visual database management.

### Reset Database

```bash
yarn prisma db push --force-reset
yarn prisma db seed
```

**Warning**: This deletes all data!

### View Logs

```bash
# Query event logs
yarn prisma db execute --stdin <<EOF
SELECT id, name, array_length(events::json, 1) as event_count, created_at 
FROM event_logs 
ORDER BY created_at DESC;
EOF
```

## Troubleshooting

### Issue: Prisma Client Not Found

```bash
yarn prisma generate
```

### Issue: Database Connection Error

Verify `DATABASE_URL` in `.env` is correct:

```bash
cat .env | grep DATABASE_URL
```

### Issue: Port 3000 Already in Use

```bash
# Kill process on port 3000
lsof -ti:3000 | xargs kill -9

# Or use a different port
PORT=3001 yarn dev
```

### Issue: Build Errors

```bash
# Clear Next.js cache
rm -rf .next

# Clear node_modules and reinstall
rm -rf node_modules yarn.lock
yarn install
```

### Issue: TypeScript Errors

```bash
# Regenerate Prisma client
yarn prisma generate

# Check TypeScript
yarn tsc --noEmit
```

## Development Tools

### Format Code

```bash
yarn format  # (if configured)
```

### Lint Code

```bash
yarn lint
```

### Type Check

```bash
yarn tsc --noEmit
```

## Configuration

Edit `config.json` to customize:

```json
{
  "app": {
    "name": "ML DevOps Platform",
    "version": "1.0.0",
    "environment": "development"
  },
  "features": {
    "eventReplay": true,
    "widgetRegistry": true,
    "streamingChat": true,
    "eventPersistence": true
  },
  "providers": {
    "ai": {
      "type": "abacus",
      "useMock": false,
      "defaultModel": "gpt-4.1-mini",
      "temperature": 0.7,
      "maxTokens": 1000
    }
  },
  "ui": {
    "theme": "system",
    "enableAnimations": true,
    "defaultReplaySpeed": 100
  }
}
```

## Deployment

### Deploy to Production

The application is containerized and deployed automatically. See deployment logs for details.

### Environment Variables for Production

Ensure these are set:

```env
DATABASE_URL=postgresql://...
ABACUSAI_API_KEY=...
NEXTAUTH_URL=https://your-domain.com
NODE_ENV=production
```

## Performance Optimization

### 1. Event Stream Size

For production, implement event log rotation:

```typescript
// In event-stream.ts
if (this.events.length > 10000) {
  // Archive old events
  await this.archiveEvents(this.events.slice(0, -1000));
  this.events = this.events.slice(-1000);
}
```

### 2. Database Indexing

Indexes are already configured in `schema.prisma`:

```prisma
@@index([createdAt])
```

### 3. React Performance

Widgets use `React.memo` and proper key management for optimal rendering.

## Next Steps

1. **Explore the Documentation**: Read [README.md](./README.md) and [ARCHITECTURE.md](./ARCHITECTURE.md)
2. **Test All Features**: Use the Event Simulator to explore system capabilities
3. **Review Code**: Study the event system implementation in `lib/events/`
4. **Plan Migration**: Review [phase2_tasks.csv](./phase2_tasks.csv) for Tauri + Rust port

## Support

For issues or questions:

1. Check this setup guide
2. Review error logs in the console
3. Inspect database with Prisma Studio
4. Check Network tab for API errors

## Development Workflow

### Typical Day-to-Day

```bash
# Start development
cd nextjs_space
yarn dev

# Make changes to code
# Auto-reload happens automatically

# Test changes
# Use Event Simulator and Chat

# Commit changes
git add .
git commit -m "Description"
git push
```

### Adding New Event Types

1. Add type to `lib/events/types.ts`
2. Update event handlers in components
3. Update documentation
4. Add simulator button (optional)
5. Test with replay

### Adding New Widgets

1. Create component in `components/widgets/`
2. Add to `widget-registry.tsx` switch statement
3. Add mount button to simulator
4. Test mount/update/unmount lifecycle
5. Update documentation

## Security Notes

- API keys are stored in `.env` (never commit to version control)
- Database credentials are managed by Abacus.AI
- Event streams may contain sensitive data - implement access control in production
- Event persistence uses PostgreSQL JSONB - validate event structure

## Known Limitations

1. **Event Stream Size**: In-memory storage limited by browser/server memory
2. **Widget Types**: Only 4 widget types currently supported
3. **AI Provider**: Only Abacus.AI provider in MVP (Rust providers coming in Phase 2)
4. **Concurrent Replay**: Only one replay can run at a time
5. **Event Filtering**: Limited filtering in current UI

## Roadmap

See [phase2_tasks.csv](./phase2_tasks.csv) for detailed migration plan.
