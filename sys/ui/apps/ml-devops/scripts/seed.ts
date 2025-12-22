import { PrismaClient } from '@prisma/client';
import bcrypt from 'bcryptjs';

const prisma = new PrismaClient();

async function main() {
  console.log('Seeding database...');

  // Create test user
  const hashedPassword = await bcrypt.hash('johndoe123', 12);
  
  await prisma.user.upsert({
    where: { email: 'john@doe.com' },
    update: {},
    create: {
      email: 'john@doe.com',
      name: 'John Doe',
      password: hashedPassword,
      role: 'admin',
    },
  });

  // Create admin user
  const adminPassword = await bcrypt.hash('Admin123!', 12);
  
  await prisma.user.upsert({
    where: { email: 'admin@mldevops.local' },
    update: {},
    create: {
      email: 'admin@mldevops.local',
      name: 'Admin User',
      password: adminPassword,
      role: 'admin',
    },
  });

  console.log('Database seeded successfully!');
}

main()
  .catch((e) => {
    console.error(e);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
